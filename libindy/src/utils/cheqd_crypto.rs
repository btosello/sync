use indy_api_types::IndyError;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use cosmos_sdk::rpc;
use prost::Message;

pub fn check_proofs(
    result: rpc::endpoint::abci_query::Response,
) -> IndyResult<()> {
    // Decode state proofs

    // Decode proof for inner ival tree
    let proof_op_0 = &result.response.proof.clone().unwrap().ops[0];
    let proof_0_data_decoded =
        ics23::CommitmentProof::decode(proof_op_0.data.as_slice()).unwrap();

    // Decode proof for outer `ics23:simple` tendermint tree)
    let proof_op_1 = &result.response.proof.unwrap().ops[1];
    let proof_1_data_decoded =
        ics23::CommitmentProof::decode(proof_op_1.data.as_slice()).unwrap();

    // Get a root hash for the inner ival tree from the outer tree proof
    let proof_1_existence = if let Some(ics23::commitment_proof::Proof::Exist(ex)) =
    proof_1_data_decoded.proof.clone()
    {
        ex
    } else {
        return Err(IndyError::from_msg(
            IndyErrorKind::InvalidStructure,
            format!(
                "Commitment proof has an incorrect format {}",
                serde_json::to_string(proof_op_1)?
            ),
        ));
    };
    let proof_0_root = proof_1_existence.clone().value;

    // Check state proofs 0 (inner iavl tree)
    let is_proof_correct = match proof_0_data_decoded.proof {
        Some(ics23::commitment_proof::Proof::Exist(_)) => {
            ics23::verify_membership(
                &proof_0_data_decoded, // proof for verification
                &ics23::iavl_spec(), // tree specification
                &proof_0_root, // value root hash in the inner ival tree (value for outer tree)
                &proof_op_0.key, // key for the inner ival tree
                &result.response.value, // received value
            )
        }
        Some(ics23::commitment_proof::Proof::Nonexist(_)) => {
            ics23::verify_non_membership(
                &proof_0_data_decoded, // proof for verification
                &ics23::iavl_spec(), // tree specification
                &proof_0_root, // value root hash in the inner ival tree
                &proof_op_0.key // key for the inner ival tree
            )
        }
        _ => {false}
    };

    if !is_proof_correct {
        return Err(IndyError::from_msg(
            IndyErrorKind::InvalidStructure,
            format!(
                "Commitment proof 0 is incorrect {}",
                serde_json::to_string(proof_op_0)?
            ),
        ));
    }

    // Should be output from light client
    // Calculate a root hash for the outer tree
    let proof_1_root = ics23::calculate_existence_root(&proof_1_existence.clone())
        .map_err(|er | IndyError::from_msg(
        IndyErrorKind::InvalidStructure,
        format!("Commitment proof has an incorrect format {}", er)))?;

    // Check state proofs 1 (outer `ics23:simple` tendermint tree)
    if !ics23::verify_membership(
        &proof_1_data_decoded, // proof for verification
        &ics23::tendermint_spec(), // tree specification
        &proof_1_root,  // root hash for the outer tree
        &proof_op_1.key, // key for the outer tree
        &proof_0_root, // inner tree root hash in the outer tree (should exist)
    ) {
        return Err(IndyError::from_msg(
            IndyErrorKind::InvalidStructure,
            format!(
                "Commitment proof 1 is incorrect {}",
                serde_json::to_string(proof_op_1)?
            ),
        ));
    }

    Ok(())
}
