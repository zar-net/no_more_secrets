use std::time::{SystemTime, UNIX_EPOCH};

// Define the structure of each block in the blockchain
#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u64,
    message: String, // User-defined message
    previous_hash: String,
}

// Implement the functionality for the Block
impl Block {
    fn new(index: usize, timestamp: u64, message: String, previous_hash: String) -> Block {
        Block {
            index,
            timestamp,
            message,
            previous_hash,
        }
    }

    fn simple_hash(&self) -> String {
        format!("{}:{}:{}:{}", self.index, self.timestamp, self.message, self.previous_hash)
    }
}

// Define the blockchain structure
struct Blockchain {
    chain: Vec<Block>,
}

// Implement the functionality for the Blockchain
impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, message: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            message,
            prev_block.simple_hash(),
        );
        self.chain.push(new_block);
    }
}

fn main() {
    let mut my_blockchain = Blockchain::new();

    // User-defined messages for each new block
    my_blockchain.add_block("First user message".to_string());
    my_blockchain.add_block("Second user message".to_string());

    println!("Blockchain: {:?}", my_blockchain.chain);
}
