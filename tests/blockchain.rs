#[cfg(test)]
mod tests {
    use addax::ConsensusAlgorithm;
    use addax::Transaction;
    use addax::{Block, Blockchain};
    use chrono::Utc;

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new("1st", "1.0.0");
        let timestamp = Utc::now();

        let block = Block::new(
            1,
            1234567890,
            500.0,
            timestamp,
            vec![Transaction::new(
                String::from("Alice"),
                String::from("Bob"),
                50.0,
                String::from("signature"),
            )],
            blockchain.get_latest_block().unwrap().hash.clone(),
            String::from("block_hash"),
        );

        // Add the block to the blockchain
        blockchain.add_block(block);

        // Verify that the blockchain contains one block
        assert_eq!(blockchain.blocks.len(), 2); // Including the genesis block

        // Retrieve the latest block from the blockchain
        let latest_block = blockchain.get_latest_block().unwrap();

        // Verify that the latest block's index is correct
        assert_eq!(latest_block.index, 1);

        // Verify that the latest block's previous_hash matches the hash of the previous block
        let previous_block = &blockchain.blocks[0];
        let previous_block_hash =
            ConsensusAlgorithm::calculate_hash(previous_block, previous_block.nonce);
        assert_eq!(latest_block.hash_, previous_block_hash);
    }
}