extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::cheqd_ledger::CheqdLedger;
use crate::libindy::cheqd_pool::CheqdPool;
use crate::libindy::cheqd_keys::CheqdKeys;

use serde_json::{Value};

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("cheqd-ledger", "Cheqd ledger management commands"));
}

pub mod query_account_command {
    use super::*;

    command!(CommandMetadata::build("query-account", "Query account for cheqd.")
                .add_required_param("address", "Address of account")
                .add_required_param("pool_alias", "Alias of pool")
                .add_example("cheqd-ledger query-account address=sov pool_alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let address = get_str_param("address", params).map_err(error_err!())?;
        let pool_alias = get_str_param("pool_alias", params).map_err(error_err!())?;

        let query = CheqdLedger::build_query_account(address)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = CheqdPool::abci_query(pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = CheqdLedger::parse_query_account_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("{}",parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod create_nym_command {
    use super::*;

    command!(CommandMetadata::build("create-nym", "Create nym.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_required_param("verkey", "Verification key")
                .add_required_param("pool_alias", "Alias of pool")
                .add_required_param("key_alias", "Alias of key")
                .add_required_param("max_coin", "Max coin for transaction")
                .add_optional_param("role", "Role of identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, ENDORSER, NETWORK_MONITOR or associated number, or empty in case of blacklisting NYM")
                .add_example("verim-ledger create-nym did=my_did verkey=my_verkey pool_alias=my_pool max_coin=500 key_alias=my_key role=TRUSTEE")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_str_param("verkey", params).map_err(error_err!())?;
        let pool_alias = get_str_param("pool_alias", params).map_err(error_err!())?;
        let key_alias = get_str_param("key_alias", params).map_err(error_err!())?;
        let role = get_str_param("role", params).map_err(error_err!())?;
        let max_coin = get_str_param("max_coin", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let key_info = CheqdKeys::get_info(wallet_handle, key_alias)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let key_info_json: Value = serde_json::from_str(&key_info).unwrap();
        let account_id = key_info_json["account_id"].as_str().unwrap();
        let pubkey = key_info_json["pub_key"].as_str().unwrap();

        let request = CheqdLedger::build_msg_create_nym(&did, account_id, verkey, pool_alias, role)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let (account_number, account_sequence) = get_base_account_number_and_sequence(account_id, pool_alias)?;

        let tx = CheqdLedger::build_tx(
            pool_alias,
            pubkey,
            &request,
            account_number,
            account_sequence,
            10000000,
            max_coin.parse::<u64>().unwrap(),
            "token",
            39039,
            "memo"
        ).map_err(|err| handle_indy_error(err, None, None, None))?;

        let signed_tx = CheqdKeys::sign(wallet_handle, key_alias, &tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = CheqdPool::broadcast_tx_commit(pool_alias, &signed_tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = CheqdLedger::parse_msg_create_nym_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("{}", parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

fn get_base_account_number_and_sequence(address: &str, pool_alias: &str) -> Result<(u64, u64), ()> {
    let query = CheqdLedger::build_query_account(address)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let response = CheqdPool::abci_query(pool_alias, &query)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response = CheqdLedger::parse_query_account_resp(&response)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response: Value = match serde_json::from_str(&parsed_response) {
        Ok(json) => json,
        Err(_) => {
            println_err!("Invalid json response. Can't parse response.");
            return Err(())
        }
    };

    if parsed_response["account"].is_null() {
        println_err!("Invalid json response. Can't get account from response.");
        return Err(());
    }
    let account = parsed_response["account"].as_object().unwrap();

    if !account.contains_key("base_account") {
        println_err!("Invalid account. Can't get base account from account.");
        return Err(());
    }
    let base_account = account["base_account"].as_object().unwrap();

    if !base_account.contains_key("account_number") {
        println_err!("Invalid base account. Can't get account number from base account.");
        return Err(());
    }
    let account_number = base_account["account_number"].as_u64().unwrap();

    if !base_account.contains_key("sequence") {
        println_err!("Invalid base account. Can't get sequence from base account.");
        return Err(());
    }
    let account_sequence = base_account["sequence"].as_u64().unwrap();

    Ok((account_number, account_sequence))
}