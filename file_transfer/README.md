
# Rust FTP-Like File Transfer

This project provides a simple implementation of a file transfer utility in Rust, which can act as both a client and a server. It's designed to transfer all files from the client's directory to the server without using the FTP protocol or its commands. Instead, it uses a simplified custom protocol over TCP for transferring files.

## Features

- **Dual Mode**: Operate as either a server or a client with a simple command-line switch.
- **Directory Transfer**: Automatically transfers all files within the client's current directory.
- **Custom Protocol**: Uses a simplified, custom file transfer protocol over TCP.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can download them from [rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Installation

1. Clone the repository to your local machine:

   ```bash
   git clone https://your-repository-url-here.git
   cd your-project-directory
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

### Running the Application

#### As a Server

To run the application in server mode, listening for incoming file transfers, use the `-S` flag:

```bash
cargo run -- -s
```

The server will start listening on `0.0.0.0:7878` for incoming connections.

#### As a Client

To run the application in client mode to transfer files to a server, specify the server's address as an argument:

```bash
cargo run -- [SERVER_IP]:7878
```

Replace `[SERVER_IP]` with the server's IP address. The client will then transfer all files from its current directory to the server.

## How It Works

- **Server Mode**: Listens for incoming connections. For each connection, it receives the filename size, filename, and file contents in sequence, saving the received files to its current directory.
- **Client Mode**: Connects to the specified server and sends all files in the current directory. Each file is sent by first sending the filename size and filename, followed by the file contents.

## Explanation

- The program checks for the -S flag. If present, it runs in server mode, listening on 0.0.0.0:7878 and waiting for incoming connections. It expects the client to send the size of the filename followed by the filename and then the file contents.

- If the -S flag is not provided, it assumes client mode, connects to the specified server address, and attempts to transfer all files in the current directory. The client reads each file, sends the filename size followed by the filename, and then sends the file content in chunks.

- This example uses async-std for file operations and tokio for networking, which are popular asynchronous runtimes in the Rust ecosystem. They allow the program to handle I/O operations without blocking, making it efficient even under high load or when dealing with large files.

