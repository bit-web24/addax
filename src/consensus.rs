use crate::{blockchain::block::Block, Transaction};
use chrono::{DateTime, Utc};
use hex;
use serde_json;
use sha2::{Digest, Sha256};

pub struct ConsensusAlgorithm;

impl ConsensusAlgorithm {
    pub fn mine_block(index: u64, coinbase: f64, ledger: &Vec<Transaction>, hash_: &String) -> Block {
        let target_difficulty = "0000";
        let mut nonce = 0;
        let mut hash = String::new();
        let mut timestamp = Utc::now();

        loop {
            let _timestamp = Utc::now();
            let _hash = ConsensusAlgorithm::calculate_hash(
                index, nonce, coinbase, _timestamp, ledger, hash_,
            );
            if hash.starts_with(target_difficulty) {
                hash = _hash;
                timestamp = _timestamp;
                break;
            }
            nonce += 1;
        }

        Block {
            index,
            nonce,
            coinbase,
            timestamp,
            ledger: ledger.to_vec(),
            hash_: hash_.to_string(),
            hash,
        }
    }

    pub fn calculate_hash(
        index: u64,
        nonce: u64,
        coinbase: f64,
        timestamp: DateTime<Utc>,
        ledger: &Vec<Transaction>,
        hash_: &String,
    ) -> String {
        let data = serde_json::json!({
            "index": index,
            "nonce": nonce,
            "coinbase": coinbase,
            "timestamp": timestamp,
            "ledger": serde_json::to_string(&ledger).unwrap(),
            "hash_": hash_,
        });
        let data = serde_json::to_string(&data).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
