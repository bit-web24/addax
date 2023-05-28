use crate::consensus::ConsensusAlgorithm;
use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
        nonce: u64,
        coinbase: f64,
        timestamp: DateTime<Utc>,
        ledger: Vec<Transaction>,
        hash_: String,
        hash: String,
    ) -> Self {
        Block {
            index,
            nonce,
            coinbase,
            timestamp,
            ledger,
            hash_,
            hash,
        }
    }
}

pub struct Blockchain {
    pub _id: String,
    pub name: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(name: &str, version: &str) -> Self {
        let timestamp = Utc::now();

        let genesis_transaction = Transaction::new(
            String::from("Genesis"),
            String::from("Genesis"),
            0.0,
            String::from("Genesis"),
        );

        let mut genesis_block = Block {
            index: 0,
            nonce: 1,
            coinbase: 1000.0,
            timestamp: timestamp,
            ledger: vec![genesis_transaction],
            hash_: String::from(""),
            hash: String::from("genesis_hash"),
        };

        ConsensusAlgorithm::mine_block(&mut genesis_block);

        let _id_json = serde_json::to_string(&serde_json::json!({
            "name": name,
            "version": version,
            "timestamp": timestamp,
        }))
        .unwrap();

        let mut hasher = Sha256::new();
        hasher.update(_id_json);
        let _id = hex::encode(hasher.finalize());

        Blockchain {
            _id,
            name: name.to_string(),
            version: version.to_string(),
            timestamp,
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        let mut prev_hash = String::from("");

        for block in &self.blocks {
            let calculated_hash = ConsensusAlgorithm::calculate_hash(&block, block.nonce);

            if block.hash_ != prev_hash {
                return false;
            }

            if block.hash != calculated_hash {
                return false;
            }

            prev_hash = block.hash.clone();
        }

        true
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}
