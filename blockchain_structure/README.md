# Simple Blockchain in Rust

This project demonstrates a basic implementation of a blockchain in Rust. It's a simplified version without encryption, focusing on the data structure of blockchain technology. Each block in the blockchain contains a user-defined message, making it possible to store and verify unique data across the chain.

## Features

- Basic block structure with index, timestamp, user-defined message, and a simple hash of the previous block.
- Simple blockchain structure to chain blocks together.
- Functionality to add blocks with user-defined messages to the blockchain.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. 

You can download Rust and find installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Running the Project

```bash
git clone git clone https://github.com/zar-net/no_more_secrets.git
cd <project-directory>
cargo run
```

### Sample Output
```
Blockchain: [
    Block { index: 0, timestamp: 1710936511, message: "Genesis Block", previous_hash: "0" }, 

    Block { index: 1, timestamp: 1710936511, message: "First user message", previous_hash: "0:1710936511:Genesis Block:0" }, 
    
    Block { index: 2, timestamp: 1710936511, message: "Second user message", previous_hash: "1:1710936511:First user message:0:1710936511:Genesis Block:0" }]
```

### How It Works
1. The blockchain is initialized with a genesis block.
2. New blocks can be added to the blockchain, each containing a user-defined message.
3. Each new block includes a 'simple hash' of the previous block, chaining them together. 

(Note: This 'simple hash' is not a secure cryptographic hash, as this example omits encryption for simplicity.)

### Disclaimer
This project is for educational purposes only. It lacks the security features and complexity of a real-world blockchain implementation.