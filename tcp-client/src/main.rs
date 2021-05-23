use std::net::{TcpStream, Shutdown};
use std::io::{Write};
use std::io::{self};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");
                let message = buffer.trim().to_string();
                stream.write(buffer.as_bytes()).unwrap();
                // if input is exit from stdin, shutdown write channel and break
                if message == "exit" {
                    //stream.shutdown(Shutdown::Both).unwrap();
                    stream.shutdown(Shutdown::Write).unwrap();
                    break;
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}