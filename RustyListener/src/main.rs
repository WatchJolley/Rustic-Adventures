use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

fn handle_client(mut stream: TcpStream, tx: Sender<String>) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    // Read data from the client
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                tx.send(message).unwrap(); // Send the received message to the main thread
            },
            _ => {
                println!("Connection closed: {}", stream.peer_addr().unwrap());
                break;
            }
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080");

    let (tx, rx): (std::sync::mpsc::Sender<String>, Receiver<String>) = channel();
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    // The move keyword is used to move ownership of the rx variable into the new thread.
    // This is necessary because Rust requires that all variables used in a closure be owned by the closure.
    let receiver_thread = thread::spawn(move || {
        for message in rx {
            println!("Received message: {}", message);
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx_clone = tx.clone(); // Clone the channel sender for each new client
                thread_vec.push(thread::spawn(move || handle_client(stream, tx_clone))); // Spawn a new thread for each new client
            },
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    receiver_thread.join().unwrap();

    for thread in thread_vec {
        thread.join().unwrap();
    }
}