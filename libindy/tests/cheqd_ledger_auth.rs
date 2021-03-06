#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use indyrs::ErrorCode;

#[cfg(feature = "cheqd")]
use utils::{cheqd_keys, cheqd_pool, cheqd_setup, cheqd_ledger};
use utils::{constants::*, types::ResponseType};
use serde_json::Value;


#[cfg(feature = "cheqd")]
mod high_cases {
    use super::*;

    #[cfg(test)]
    mod build_tx {
        use super::*;

        #[test]
        fn test_build_tx() {
            let setup = cheqd_setup::CheqdSetup::new();

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Tx
            let tx = cheqd_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0, "cheq", setup.get_timeout_height(), "memo",
            ).unwrap();

            println!("Tx: {:?}", tx);
            assert_ne!(tx.len(), 0);
        }
    }

    #[cfg(test)]
    mod query_account {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_query_account() {
            let setup = cheqd_setup::CheqdSetup::new();

            let query = cheqd_ledger::auth::build_query_account(&setup.account_id).unwrap();
            let resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let parsed = cheqd_ledger::auth::parse_query_account_resp(&resp).unwrap();

            println!("Parsed query response: {:?}", parsed);
        }
    }
}
