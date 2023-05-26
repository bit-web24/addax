use serde::{Deserialize, Serialize};
// use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        data: String,
        previous_hash: String,
        hash: String,
    ) -> Self {
        Block {
            index,
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
            String::from("Genesis Block"),
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
