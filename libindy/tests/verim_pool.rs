#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use indyrs::ErrorCode;

use utils::{constants::*, verim_keys, auth, verim_pool, verim_setup, verim_ledger, types::ResponseType};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_pool {
        use super::*;

        #[test]
        fn test_add() {
            let result = verim_pool::add("pool1", "rpc_address", "chain_id").unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod get_pool {
        use super::*;

        #[test]
        fn test_get() {
            verim_pool::add("pool1", "rpc_address", "chain_id").unwrap();
            let result = verim_pool::get_config("pool1").unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod broadcast_tx_commit {
        use super::*;

        #[test]
        fn test_broadcast_tx_commit() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.alice_account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature
            let signed = verim_keys::sign(&setup.alice_key_alias, &tx).unwrap();
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();
            verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci{
        use super::*;

        #[test]
        fn test_abci_query() {
            unimplemented!();
        }

        #[test]
        fn test_abci_info() {
            unimplemented!();
        }
    }

}
