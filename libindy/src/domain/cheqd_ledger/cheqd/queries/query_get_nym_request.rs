use indy_api_types::errors::IndyResult;

use super::super::super::proto::cheqdid::cheqdnode::cheqd::QueryGetNymRequest as ProtoQueryGetNymRequest;
use super::super::super::CheqdProto;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryGetNymRequest {
    pub id: u64,
}

impl QueryGetNymRequest {
    pub fn new(id: u64) -> Self {
        QueryGetNymRequest { id }
    }
}

impl CheqdProto for QueryGetNymRequest {
    type Proto = ProtoQueryGetNymRequest;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self {
            id: proto.id.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_get_nym_request() {
        let msg = QueryGetNymRequest::new(456);

        let proto = msg.to_proto();
        let decoded = QueryGetNymRequest::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
