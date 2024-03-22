// Import necessary libraries and modules
use aes::{Aes128, cipher::{NewCipher, BlockEncrypt, BlockDecrypt, generic_array::GenericArray}};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use hex::{encode, decode};

fn main() {
    // Parse command-line arguments to determine if the program should run as a server or client
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: program [server|client]");
        return;
    }

    match args[1].as_str() {
        "server" => start_server(),
        "client" => start_client(),
        _ => println!("Invalid argument. Use 'server' or 'client'."),
    }
}

fn start_server() {
    // Generate RSA private and public keys for the server
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    // Start the TCP server and listen for incoming connections
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let public_key = public_key.clone();
        match stream {
            Ok(stream) => {
                // Handle each client connection in a separate thread
                thread::spawn(move || handle_client(stream, private_key.clone(), public_key));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, private_key: RsaPrivateKey, public_key: RsaPublicKey) {
    let mut buffer = [0; 2048];

    // Send the server's public RSA key to the client for asymmetric encryption of the symmetric key
    println!("Server sending public RSA key to client for asymmetric encryption");
    let pub_key_bytes = public_key.to_pkcs1().unwrap();
    stream.write_all(&pub_key_bytes).unwrap();

    // Receive the symmetric key from the client, encrypted with the server's public RSA key
    let _ = stream.read(&mut buffer).unwrap();
    let encrypted_symmetric_key = private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &buffer[..]).unwrap();

    // Use the decrypted symmetric key for further encryption/decryption (AES-128 in this example)
    let symmetric_key = GenericArray::from_slice(&encrypted_symmetric_key[..16]);  // Using only the first 16 bytes for AES-128

    // Receive an encrypted message from the client, encrypted using the symmetric key
    let _ = stream.read(&mut buffer).unwrap();
    let encrypted_data = decode(&String::from_utf8_lossy(&buffer[..])).unwrap();
    let mut decrypted_data = vec![0u8; encrypted_data.len()];
    let cipher = Aes128::new(&symmetric_key);

    // Decrypt the message using AES-128 with the symmetric key
    for (chunk, decrypted_chunk) in encrypted_data.chunks(16).zip(decrypted_data.chunks_mut(16)) {
        let block = GenericArray::from_slice(chunk);
        let decrypted_block = cipher.decrypt_block(&block);
        decrypted_chunk.copy_from_slice(decrypted_block.as_slice());
    }

    println!("Server decrypted: {}", String::from_utf8(decrypted_data).unwrap());
}

/// The client first receives the server's public RSA key, then encrypts a symmetric AES key with 
/// it and sends the encrypted symmetric key to the server. Next, it encrypts a message using 
/// the symmetric AES key and sends the encrypted message to the server.
/// 
fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            let mut buffer = [0; 2048];

            // Receive the server's public RSA key
            stream.read(&mut buffer).unwrap();
            let server_public_key = RsaPublicKey::from_pkcs1(&buffer[..]).unwrap();
            println!("Client received server's public RSA key");

            // Encrypt a symmetric key (AES-128) with the server's public RSA key
            let symmetric_key = b"an_example_16byte"; // 16 bytes for AES-128
            let mut rng = OsRng;
            let encrypted_symmetric_key = server_public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), symmetric_key).unwrap();
            println!("Client encrypting and sending symmetric key with server's public RSA key");
            stream.write_all(&encrypted_symmetric_key).unwrap();

            // Encrypt a message with the symmetric key using AES-128
            let message = b"Hello, this is a test message!";
            let cipher = Aes128::new(GenericArray::from_slice(symmetric_key));
            let mut encrypted_message = vec![0u8; message.len()]; // Initialize a vector to hold the encrypted message
            let mut decrypted_message = vec![0u8; message.len()]; // Initialize a vector to hold the decrypted message (for comparison)

            // Encrypt the message block by block
            for (chunk, encrypted_chunk) in message.chunks(16).zip(encrypted_message.chunks_mut(16)) {
                let block = GenericArray::from_slice(chunk);
                let encrypted_block = cipher.encrypt_block(&block);
                encrypted_chunk.copy_from_slice(encrypted_block.as_slice());
            }

            // Send the encrypted message to the server
            println!("Client sending encrypted message to server");
            stream.write_all(&encode(&encrypted_message)).unwrap(); // The encrypted message is hex encoded for transmission

            // Optionally, read server's response or perform further communication as needed
            // This part is omitted for brevity
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

