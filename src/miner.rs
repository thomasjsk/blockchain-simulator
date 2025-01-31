use crate::block::{calculate_hash, Block};
use crate::blockchain::Blockchain;
use rand;
use rand::Rng;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub struct Miner {
    pub id: u32,
    pub block_mined_tx: Sender<(Block, u32)>,
    pub coins: u32,
}

impl Miner {
    pub fn new(id: u32, block_mined_tx: Sender<(Block, u32)>) -> Self {
        Miner {
            id,
            block_mined_tx,
            coins: 0,
        }
    }

    pub fn mine_block(&self, blockchain: Blockchain, terminator: Receiver<()>) -> JoinHandle<()> {
        let miner_id = self.id;
        let block_mined_tx = self.block_mined_tx.clone();

        thread::spawn(move || {
            // println!("Miner #{} is mining...", miner_id);

            // Get the latest blockchain state
            let prev_block = blockchain.chain.last().unwrap();
            let block_data = blockchain.mempool.join(",");

            let mut nonce = rand::rng().random(); // Generate random nonce
            loop {
                if terminator.try_recv().is_ok() {
                    // println!("Miner #{} aborting mining process", miner_id);
                    break;
                }

                // Attempt to mine a block
                if let Some(block) = Miner::mine(
                    prev_block.index + 1,
                    block_data.clone(),
                    prev_block.hash.clone(),
                    blockchain.difficulty,
                    nonce,
                ) {
                    println!("Miner #{} found the block", miner_id);
                    block_mined_tx.send((block, miner_id)).unwrap();

                    break;
                }
                nonce += 1;
            }
        })
    }

    fn mine(
        index: u32,
        data: String,
        previous_hash: String,
        difficulty: u32,
        nonce: u64,
    ) -> Option<Block> {
        let timestamp = chrono::Utc::now().timestamp() as u64;

        let hash = calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        if hash.starts_with(&"0".repeat(difficulty as usize)) {
            return Some(Block {
                index,
                timestamp,
                data,
                previous_hash,
                hash,
                nonce,
            });
        }

        None
    }
}
