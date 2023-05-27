use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub coinbase: f64,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        nonce: u64,
        coinbase: f64,
        timestamp: u64,
        data: Vec<Transaction>,
        previous_hash: String,
        hash: String,
    ) -> Self {
        Block {
            index,
            nonce,
            coinbase,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            0,
            1000.0,
            0,
            vec![Transaction {
                sender: String::from("Genesis"),
                receiver: String::from("Genesis"),
                amount: 0.0,
                transaction_id: String::from("Genesis"),
                timestamp: 0,
                additional_data: String::from("Genesis"),
            }],
            String::from(""),
            String::from("genesis_hash"),
        );
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
