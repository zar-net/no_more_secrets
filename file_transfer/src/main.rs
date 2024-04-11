//use clap::{App, Arg};
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use std::env;
use async_std::prelude::*;
use async_std::fs::File;
use async_std::fs;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


const BUFFER_SIZE: usize = 4096;

async fn receive_files(mut socket: TcpStream) -> async_std::io::Result<()> {
    loop {
        let mut filename_size_buf = [0u8; 2];
        if socket.read_exact(&mut filename_size_buf).await.is_err() {
            break; // End the loop if we can't read (e.g., connection closed)
        }
        let filename_size = u16::from_be_bytes(filename_size_buf);

        if filename_size == 0 {
            break; // If filename size is 0, no more files are being sent
        }

        let mut filename_buf = vec![0u8; filename_size as usize];
        socket.read_exact(&mut filename_buf).await?;
        let filename = String::from_utf8_lossy(&filename_buf);

        let mut file = File::create(&*filename).await?;
        println!("Receiving file {}", filename);

        let mut buffer = [0u8; BUFFER_SIZE];
        while let Ok(n) = socket.read(&mut buffer).await {
            if n == 0 { break; } // Break if no more data for the current file
            file.write_all(&buffer[..n]).await?;
        }
    }
    Ok(())
}

async fn run_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    let (socket, _) = listener.accept().await?;

    let _ = receive_files(socket).await;

    /* 
    while let Ok(filename_size) = socket.read_u16().await {
        let mut filename_buf = vec![0u8; filename_size as usize];
        socket.read_exact(&mut filename_buf).await?;
        let filename = String::from_utf8_lossy(&filename_buf);

        let mut file = File::create(filename.to_string()).await?;
        println!("Receiving file {}", filename.to_string());

        let mut buffer = [0u8; BUFFER_SIZE];

        while let Ok(n) = socket.read(&mut buffer).await {
            if n == 0 { break; }
            file.write_all(&buffer[..n]).await?;
        }
    }
    */

    Ok(())
}

async fn run_client(addr: &str, dir: &str) -> tokio::io::Result<()> {
    let mut paths = fs::read_dir(dir).await?;
    let mut connection = TcpStream::connect(addr).await?;

    //for path in paths {
    while let Some(res) = paths.next().await {
        let path = res?.path();
        let metadata = fs::metadata(&path).await?;
        
        if metadata.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();

            println!("Sending file {}", filename.to_string());

            let mut file = File::open(&path).await?;

            connection.write_u16(filename.len() as u16).await?;
            connection.write_all(filename.as_bytes()).await?;

            let mut buffer = [0u8; BUFFER_SIZE];
            while let Ok(n) = file.read(&mut buffer).await {
                if n == 0 { break; }
                connection.write_all(&buffer[..n]).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    
    let matches = Command::new("Rust FTP-Like File Transfer")
        .version("1.0")
        .author("Bryan Zarnett")
        .about("Transfers files from client to server")
        .arg(Arg::new("server")
            .short('s')
            .long("server")
            .action(ArgAction::SetTrue)
            .help("Runs the program in server mode"))
        .arg(Arg::new("ADDRESS")
            .help("Sets the server address")
            .required_unless_present("server")
            .index(1))
        .get_matches();

    if matches.get_flag("server") {
        println!("Running as file transfer server.");
        run_server("0.0.0.0:7878").await?;
    } else {
        println!("Running as file transfer client.");
        let addr = matches.get_one::<String>("ADDRESS").expect("ADDRESS is required");
        let current_dir = env::current_dir().unwrap();
        run_client(addr, current_dir.to_str().unwrap()).await?;
    }

    Ok(())
}
