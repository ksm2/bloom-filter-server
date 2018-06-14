// Define dependencies
extern crate bit_vec;
extern crate murmur3;

// Main
use bloom_filter::BloomFilter;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

// Define Modules
mod bloom_filter;

fn main() {
    let (parent_tx, rx) = channel::<(String, Sender<Vec<u8>>)>();

    let host = "0.0.0.0";
    let port = 1337;
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening for connections on address tcp://{}", addr);

    thread::spawn(move || {
        handle_server(rx)
    });

    let mut i = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let cloned_tx = parent_tx.clone();
                i += 1;
                let my_id = i;
                thread::spawn(move || {
                    handle_client(my_id, cloned_tx, stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

/// Handles TCP streams from clients
fn handle_client(id: i32, parent_tx: Sender<(String, Sender<Vec<u8>>)>, mut stream: TcpStream) -> () {
    println!("Connected to client #{}", id);

    loop {
        let mut buf = [32u8; 4096];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break
        }

        let input = String::from_utf8_lossy(&buf).trim_right().to_string();
        if input.len() == 0 {
            break
        }

        let (child_tx, parent_rx) = channel::<Vec<u8>>();
        parent_tx.send((input, child_tx)).unwrap();
        let to_send = parent_rx.recv().unwrap();
        stream.write(to_send.as_slice()).unwrap();

        thread::sleep(Duration::from_millis(1))
    }

    println!("Closed connection to client #{}", id);
}

/// Handles messages from incoming clients and holds the Bloom filter
fn handle_server(rx: Receiver<(String, Sender<Vec<u8>>)>) {
    let mut bf = BloomFilter::new();

    for (mut message, tx) in rx.iter() {
        let mut parts = message.split_whitespace();

        let method = parts.next().unwrap();
        match method.to_uppercase().as_str() {
            "ADD" => {
                for token in parts {
                    bf.add(token);
                    println!("Added '{}'", token);
                }
                tx.send(b"OK.\n".to_vec()).unwrap();
            }
            "HAS" => {
                let token = parts.next().unwrap();
                let is_contained = bf.has(token);
                println!("Check if '{}' is contained: {}", token, is_contained);
                if is_contained {
                    tx.send(b"Yes.\n".to_vec()).unwrap();
                } else {
                    tx.send(b"No.\n".to_vec()).unwrap();
                };
            }
            "BITS" => {
                let bytes = bf.to_bytes();
                tx.send(bytes).unwrap();
            }
            _ => {
                println!("Error: {}", message);
                tx.send(b"ERROR.\n".to_vec()).unwrap();
            }
        }
    }
}
