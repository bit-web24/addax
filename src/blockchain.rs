use crate::consensus::ConsensusAlgorithm;
use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

pub mod block;
use block::Block;

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

        let genesis_block = Block::new(0, 1000.0, vec![genesis_transaction], String::from(""));

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
            let calculated_hash = ConsensusAlgorithm::mine_block(
                block.index,
                block.coinbase,
                &block.ledger,
                &block.hash_,
            )
            .hash;

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
