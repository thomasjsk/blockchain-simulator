use chrono;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let nonce = 0;
        let hash = calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    // pub fn mine(index: u32, data: String, previous_hash: String, difficulty: u32) -> Block {
    //     let timestamp = chrono::Utc::now().timestamp() as u64;
    //     let mut nonce = 0;
    //
    //     println!("Mining for block: {}", index);
    //     loop {
    //         let hash = calculate_hash(index, timestamp, &data, &previous_hash, nonce);
    //
    //         if hash.starts_with(&"0".repeat(difficulty as usize)) {
    //             println!("Mined successfully for nonce: {}", nonce);
    //             return Block {
    //                 index,
    //                 timestamp,
    //                 data,
    //                 previous_hash,
    //                 hash,
    //                 nonce,
    //             };
    //         }
    //
    //         nonce += 1
    //     }
    // }
}

pub fn calculate_hash(
    index: u32,
    timestamp: u64,
    data: &str,
    previous_hash: &str,
    nonce: u64,
) -> String {
    let hash_input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
    let hash = Sha256::digest(hash_input.as_bytes());
    hex::encode(hash)
}
