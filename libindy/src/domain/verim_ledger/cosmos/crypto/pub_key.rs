//! Helper class to handle private keys generic proto conversion

use super::secp256k1;
use crate::domain::verim_ledger::prost_ext::ProstMessageExt;
use crate::domain::verim_ledger::VerimProto;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PubKey {
    Secp256k1(secp256k1::PubKey),
}

impl VerimProto for PubKey {
    type Proto = prost_types::Any;

    fn to_proto(&self) -> Self::Proto {
        unimplemented!()
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        match &proto.type_url[..] {
            "secp256k" => {
                let val = secp256k1::PubKey::from_proto_bytes(&proto.value)?;
                Ok(PubKey::Secp256k1(val))
            }
            _ => Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                "Unknown pub_key type",
            )),
        }
    }
}
