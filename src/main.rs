mod block;
mod blockchain;
mod cli;
mod miner;
mod miners_pool;

use crate::block::Block;
use clap::{Parser, Subcommand};
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

#[derive(Parser)]
#[command(name = "Simple Blockchain")]
#[command(about = "A simple blockchain simulation in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Simulate,
}

fn main() {
    // let blockchain = Arc::new(Mutex::new(blockchain::Blockchain::new()));
    let mut blockchain = blockchain::Blockchain::new();

    let (block_mined_tx, block_mined_rx): (Sender<(Block, u32)>, Receiver<(Block, u32)>) =
        mpsc::channel();

    let miner_pool = Arc::new(Mutex::new(miners_pool::MinersPool::new(block_mined_tx)));

    println!("Blockchain simulator started. Type commands below (e.g., 'sim --blocks=5 --difficulty=4'). Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting simulator. Goodbye!");
            break;
        }

        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        let mut clap_args = vec!["blockchain".to_string()];
        clap_args.extend(args);

        match cli::setup_cli(&clap_args) {
            Ok(matches) => match matches.subcommand() {
                Some(("sim", _matches)) => {
                    let difficulty: u32 = _matches
                        .get_one::<String>("difficulty")
                        .unwrap_or(&"4".to_string())
                        .parse::<u32>()
                        .expect("Invalid difficulty");

                    let blocks: u32 = _matches
                        .get_one::<String>("blocks")
                        .unwrap_or(&"1".to_string())
                        .parse::<u32>()
                        .expect("Invalid blocks number");

                    println!("Simulating mining with difficulty {}...", difficulty);
                    blockchain.difficulty = difficulty;

                    for i in 0..blocks {
                        blockchain.mempool = vec!["foo".to_string(), "bar".to_string()];
                        miner_pool.lock().unwrap().start_mining(blockchain.clone());
                        println!("Mining block: {} of {}", i, blocks);
                        loop {
                            if let Ok((block, miner_id)) = block_mined_rx.try_recv() {
                                if block.index == blockchain.chain.last().unwrap().index {
                                    continue;
                                }

                                println!("Block was mined: {:?}", block.index);
                                miner_pool.lock().unwrap().stop_mining();
                                miner_pool.lock().unwrap().reward(miner_id);

                                blockchain.mempool = vec![];
                                blockchain.add_block(block);
                                println!("\nValid: {}", blockchain.is_valid());
                                println!("Blocks mined: {}", blockchain.chain.len());
                                println!(">>Blocks");
                                println!("...");
                                let start = if blockchain.chain.len() >= 5 {
                                    blockchain.chain.len() - 5
                                } else {
                                    0
                                };
                                for i in start..&blockchain.chain.len() - 1 {
                                    let block = &blockchain.chain[i];
                                    println!("Index: {}", block.index);
                                    println!("Hash: {}", block.hash);
                                    println!("Nonce: {}", block.nonce);
                                    println!();
                                }
                                println!("Miners:");
                                for miner in &miner_pool.lock().unwrap().miners {
                                    println!("#{:2}; coins: {}", miner.id, miner.coins);
                                }

                                break;
                            }

                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                }
                Some(("miners", _matches)) => {
                    if let Some((cmd_string, sub_matches)) = _matches.subcommand() {
                        if cmd_string == "add" {
                            let miners_to_add: u32 = match sub_matches.get_one::<String>("number") {
                                Some(n) => n.parse::<u32>().unwrap_or(1),
                                None => 1,
                            };

                            println!("Adding miners: {:?}", miners_to_add);
                            miner_pool.lock().unwrap().add_miners(miners_to_add);
                        }

                        if cmd_string == "remove" {
                            if let Some(id) = sub_matches.get_one::<String>("id") {
                                let miner_id: u32 = match id.as_str() {
                                    "first" => match miner_pool.lock().unwrap().miners.first() {
                                        Some(miner) => miner.id,
                                        None => 0,
                                    },
                                    "last" => match miner_pool.lock().unwrap().miners.last() {
                                        Some(miner) => miner.id,
                                        None => 0,
                                    },
                                    _id => _id.parse::<u32>().unwrap_or_else(|_| 0),
                                };

                                println!("Removing miner #{:?}", miner_id);
                                miner_pool.lock().unwrap().remove_miner(miner_id);
                            }
                        }
                    }

                    println!(
                        "Miners pool size: {}",
                        miner_pool.lock().unwrap().miners.len()
                    );
                }
                Some(("status", _)) => {
                    println!("\n\n___________________________ Blockchain status ___________________________");
                    println!(
                        "Miners pool size: {}",
                        miner_pool.lock().unwrap().miners.len()
                    );
                    println!("\nValid: {}", blockchain.is_valid());
                    println!("Blocks mined: {}", blockchain.chain.len());
                    println!(">>Blocks");
                    for block in &blockchain.chain {
                        println!("Index: {}", block.index);
                        println!("Hash: {}", block.hash);
                        println!("Nonce: {}", block.nonce);
                        println!();
                    }
                    println!(
                        "_________________________________________________________________________"
                    );
                }
                cmd => {
                    match cmd {
                        Some(x) => println!("Invalid command: {}", x.0.to_string()),
                        None => (),
                    };
                }
            },
            Err(e) => println!("Failed to parse command: {}", e),
        }
    }
}
