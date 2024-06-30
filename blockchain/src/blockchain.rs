use crate::block::Block;
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub voters: HashMap<String, HashSet<String>>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            voters: HashMap::new(),
        };

        // Create the genesis block
        let genesis_block = Block::new(0, String::from(""), String::from(""), String::from(""), String::from("0"));
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn add_block(&mut self, voter_id: String, election_id: String, vote_option_id: String) -> Result<(), String> {
        if let Some(voters) = self.voters.get(&election_id) {
            if voters.contains(&voter_id) {
                return Err("Voter has already voted in this election".to_string());
            }
        } else {
            self.voters.insert(election_id.clone(), HashSet::new());
        }

        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            voter_id.clone(),
            election_id.clone(),
            vote_option_id,
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
        self.voters.get_mut(&election_id).unwrap().insert(voter_id);

        Ok(())
    }
}
