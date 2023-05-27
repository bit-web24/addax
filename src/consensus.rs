use crate::blockchain::Block;
use hex;
use serde_json;
use sha2::{Digest, Sha256};

pub struct ConsensusAlgorithm;

impl ConsensusAlgorithm {
    pub fn mine_block(block: &mut Block) {
        let target_difficulty = "0000";
        let mut nonce = 0;

        loop {
            let hash = ConsensusAlgorithm::calculate_hash(block, nonce);
            if hash.starts_with(target_difficulty) {
                block.hash = hash;
                block.nonce = nonce;
                break;
            }
            nonce += 1;
        }
    }

    fn calculate_hash(block: &Block, nonce: u64) -> String {
        let data = serde_json::json!({
            "index": block.index,
            "coinbase": block.coinbase,
            "timestamp": block.timestamp,
            "data": serde_json::to_string(&block.data).unwrap(),
            "previous_hash": block.previous_hash,
            "nonce": nonce
        });
        let data = serde_json::to_string(&data).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
