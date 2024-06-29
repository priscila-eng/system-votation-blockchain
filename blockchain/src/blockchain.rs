use crate::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
        };

        // Create the genesis block
        let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
    }
}
