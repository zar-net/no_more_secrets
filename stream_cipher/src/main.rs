//! Creating a stream cipher in Rust
//! 
//! A stream cipher involves generating a pseudo-random stream of bits (or characters) that 
//! can be XORed with your plaintext (the sentence) to produce ciphertext. This example will demonstrate
//! how to turn a sentence into ciphertext using a basic pseudo-random generator seeded with a fixed value.
//! 

use rand::{Rng, SeedableRng}; // rand is a commonly used Rust crate for random number generation
use rand::rngs::StdRng;

fn main() {
    // The sentence to encrypt
    let sentence = "This is a secret message.";

    // Seed for our pseudo-random number generator (PRNG)
    // In a real application, you'd want this to be more secure, possibly using an external entropy source
    let seed: u64 = 123456;

    // Create a seeded PRNG
    let mut rng = StdRng::seed_from_u64(seed);

    // Convert the sentence to bytes
    let sentence_bytes = sentence.as_bytes();

    // Generate a pseudo-random byte stream the same length as the sentence
    let mut keystream = Vec::new();
    for _ in 0..sentence_bytes.len() {
        keystream.push(rng.gen::<u8>());
    }

    // XOR the sentence with the keystream to produce the ciphertext
    let ciphertext: Vec<u8> = sentence_bytes.iter()
                                            .zip(keystream.iter())
                                            .map(|(&byte_sentence, &byte_keystream)| byte_sentence ^ byte_keystream)
                                            .collect();

    println!("Ciphertext (as bytes): {:?}", ciphertext);

    // To decrypt, XOR the ciphertext with the same keystream
    // Note: In a real cipher, the receiver must have the same keystream, which usually means sharing a secret key securely
    let decrypted_text: Vec<u8> = ciphertext.iter()
                                             .zip(keystream.iter())
                                             .map(|(&byte_ciphertext, &byte_keystream)| byte_ciphertext ^ byte_keystream)
                                             .collect();

    println!("Decrypted text: {}", String::from_utf8_lossy(&decrypted_text));
}

