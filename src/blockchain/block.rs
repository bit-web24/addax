use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{transaction::Transaction, ConsensusAlgorithm};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub coinbase: f64,
    pub timestamp: DateTime<Utc>,
    pub ledger: Vec<Transaction>,
    pub hash_: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        coinbase: f64,
        ledger: Vec<Transaction>,
        hash_: String,
    ) -> Self {
        ConsensusAlgorithm::mine_block(index, coinbase, &ledger, &hash_)
    }
}