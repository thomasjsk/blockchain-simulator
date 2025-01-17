use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::miner::Miner;

use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct MinersPool {
    pub miners: Vec<Miner>,
    mining_terminators: Vec<Sender<()>>,
    block_mined_tx: Sender<Block>,
}

impl MinersPool {
    pub fn new(block_mined_tx: Sender<Block>) -> Self {
        MinersPool {
            miners: vec![],
            mining_terminators: vec![],
            block_mined_tx,
        }
    }

    pub fn start_mining(&mut self, blockchain: Blockchain) {
        for i in 0..self.miners.len() {
            let miner = &self.miners[i];
            let (terminator_tx, terminator_rx) = mpsc::channel::<()>();
            self.mining_terminators.push(terminator_tx);
            miner.mine_block(blockchain.clone(), terminator_rx);
        }
    }

    pub fn stop_mining(&mut self) {
        while self.mining_terminators.len() > 0 {
            match &self.mining_terminators.pop() {
                Some(terminator) => {
                    terminator.send(()).unwrap_or_default();
                }
                None => (),
            };
        }
        self.mining_terminators.clear();
    }

    pub fn add_miners(&mut self, n: u32) {
        for _ in 0..n {
            self.add_miner();
        }
    }

    pub fn add_miner(&mut self) {
        let id = self.miners.len() as u32 + 1;

        let miner = Miner::new(id, self.block_mined_tx.clone());
        self.miners.push(miner);

        println!("Miner #{} added to the pool.", id)
    }

    pub fn remove_miner(&mut self, miner_id: u32) {
        println!("Miner #{} about to be removed...", miner_id);

        if miner_id == 0 {
            if let Some(_) = self.miners.pop() {
                println!("Miner #{} removed from the pool.", miner_id);
            } else {
                println!("Miner #{} was not found in the pool.", miner_id);
            }

            return;
        }

        if let Some(index) = self.miners.iter().position(|miner| miner.id == miner_id) {
            self.miners.remove(index);

            println!("Miner #{} removed from the pool.", miner_id);
        } else {
            println!("Miner #{} was not found in the pool.", miner_id);
        }

        return;
    }
}
