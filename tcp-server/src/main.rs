use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 100]; // using 100 byte buffer
    while match stream.read(&mut buffer) {
        Ok(_) => {
            // echo everything!
            let data = from_utf8(&buffer).unwrap().trim().replace("\n", "");
            println!("Receive data: {} from {} ", data, stream.peer_addr().unwrap());
            let sub:String = data.chars().skip(0).take(4).collect();
            // if read exit from client, just return
            if sub == "exit" {
                println!("exit!");
                //stream.shutdown(Shutdown::Read).unwrap();
                return;
            }
            // flush the stream
            stream.flush().ok();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Read).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let handle = thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
                thread_vec.push(handle);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
           }
        }
    }

    // wait for thread finishing task
    for handle in thread_vec {
        handle.join().unwrap();
    }

    // close the socket server
    drop(listener);
}