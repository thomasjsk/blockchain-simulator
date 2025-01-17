use crate::block;
use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub mempool: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis".to_string(), "0".to_string());

        Blockchain {
            chain: vec![genesis_block],
            difficulty: 1,
            mempool: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) -> u32 {
        println!("Adding new block #{:?}", block.index);
        let block_index = block.index.clone();
        self.chain.push(block);
        block_index
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            let recalculated_hash = block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.data,
                &current_block.previous_hash,
                current_block.nonce,
            );

            if current_block.hash != recalculated_hash {
                return false;
            }
        }

        true
    }
}
