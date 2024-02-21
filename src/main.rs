use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
 struct Block {
     index: u64,
     timestamp: DateTime<Utc>,
     data: String,
     previous_hash: String,
     hash: String,
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain { chain: vec![] }
    }

    fn add_block(&mut self, data: String) {
        let index = (self.chain.len() as u64) + 1;
        let timestamp = Utc::now();
        let previous_hash = self.chain.last().map_or(String::from("0"), |block| block.hash.clone());
        let hash = Blockchain::calculate_hash(index, &timestamp, &data, &previous_hash);
        let block = Block { index, timestamp, data, previous_hash, hash };
        self.chain.push(block);
    }

    fn calculate_hash(index: u64, timestamp: &DateTime<Utc>, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_rfc3339());
        hasher.update(data);
        hasher.update(previous_hash);
        format!("{:x}", hasher.finalize())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("First block"));
    blockchain.add_block(String::from("Second block"));

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
