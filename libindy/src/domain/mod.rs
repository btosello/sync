pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pairwise;
pub mod pool;
pub mod cache;
pub mod verim_keys;
pub mod verim_pool;
pub mod verim_ledger;

use indy_api_types::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndyConfig {
    pub crypto_thread_pool_size: Option<usize>,
    pub collect_backtrace: Option<bool>,
    pub freshness_threshold: Option<u64>
}

impl Validatable for IndyConfig {}