# Rust Stream Cipher Example

This project demonstrates a simple stream cipher in Rust for educational purposes. It shows how to encrypt and decrypt a sentence using a pseudo-random keystream generated with a fixed seed. The Rust code initializes a pseudo-random number generator with a fixed seed and generates a keystream. It then encrypts a hard-coded sentence by XORing each byte with the corresponding byte in the keystream. The decryption process is also demonstrated by XORing the ciphertext with the same keystream.

## Getting Started

To get started with this project, ensure you have Rust and Cargo installed on your system. If you haven't installed Rust, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Prerequisites

- Rust
- Cargo (comes with Rust)

### Installation

Clone this repository to your local machine using the following command:

```bash
git clone https://github.com/zar-net/stream_cipher.git
cd your-project-directory-name
```

### Dependencies

Ensure you have the rand crate as a dependency in your Cargo.toml file:

```[dependencies]
rand = "0.8"
```

### Running the Code
Navigate to the root of the project directory, where the Cargo.toml file is located. You can run the project with the following command:

```cargo run```


### What does this code do?
1. Converts the input sentence into a byte array.
2. Initializes a pseudo-random number generator (StdRng) with a fixed seed. In a real-world scenario, the seed should be a secure, unpredictable value.
3. Generates a keystream of pseudo-random bytes the same length as the input sentence.
4. XORs each byte of the input sentence with the corresponding byte from the keystream to produce the ciphertext.
5. Demonstrates decryption by XORing the ciphertext with the same keystream.

### Disclaimer
This code is for educational purposes only and demonstrates basic concepts of a stream cipher. It is not secure and should not be used in production or for protecting sensitive data.