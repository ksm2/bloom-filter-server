#![feature(test)]
#![feature(try_trait)]

// Define dependencies
extern crate rayon;
extern crate bit_vec;
extern crate murmur3;
extern crate test;

// Main
use bloom_filter::BloomFilter;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

// Define Modules
mod bloom_filter;
mod tests;

fn main() {
    let (parent_tx, rx) = channel::<([u8; 4096], Sender<Vec<u8>>)>();

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
fn handle_client(id: i32, parent_tx: Sender<([u8; 4096], Sender<Vec<u8>>)>, mut stream: TcpStream) -> () {
    println!("Connected to client #{}", id);

    loop {
        let mut buf = [32u8; 4096];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break
        }

        let (child_tx, parent_rx) = channel::<Vec<u8>>();
        parent_tx.send((buf, child_tx)).unwrap();
        let to_send = parent_rx.recv().unwrap();
        stream.write(to_send.as_slice()).unwrap();

        thread::sleep(Duration::from_millis(1))
    }

    println!("Closed connection to client #{}", id);
}

/// Handles messages from incoming clients and holds the Bloom filter
fn handle_server(rx: Receiver<([u8; 4096], Sender<Vec<u8>>)>) {
    let mut bf = BloomFilter::new();

    for (mut message, tx) in rx.iter() {
        match handle_message(&mut bf, message) {
            Ok(to_send) => tx.send(to_send).unwrap(),
            Err(_) => tx.send(b"ERROR. Unkown error.".to_vec()).unwrap(),
        }
    }
}

/// Handles an incoming message.
fn handle_message(bf: &mut BloomFilter, message: [u8; 4096]) -> Result<Vec<u8>, std::option::NoneError> {
    let command = message.get(0..3)?;
    let rest = message.get(4..)?;
    let string = String::from_utf8_lossy(rest).trim_right().to_string();
    let mut parts = string.split_whitespace();

    match command {
        b"ADD" | b"add" => {
            let tokens = parts.collect::<Vec<&str>>();
            bf.add(tokens);
            Ok(b"OK.\n".to_vec())
        }
        b"RMV" | b"rmv" => {
            for token in parts {
                bf.remove(token);
                println!("Removed '{}'", token);
            }
            Ok(b"OK.\n".to_vec())
        }
        b"HAS" | b"has" => {
            let token = parts.next()?;
            let is_contained = bf.has(token);
            println!("Check if '{}' is contained: {}", token, is_contained);
            if is_contained {
                Ok(b"Yes.\n".to_vec())
            } else {
                Ok(b"No.\n".to_vec())
            }
        }
        b"CNT" | b"cnt" => {
            let token = parts.next()?;
            let count = bf.count(token);
            Ok(format!("{}.\n", count).into_bytes())
        }
        b"BIN" | b"bin" => {
            Ok(bf.to_bytes())
        }
        _ => {
            println!("Error with incoming message.");
            Ok(format!("ERROR. Invalid command {}.\n", String::from_utf8_lossy(command).trim_right()).as_bytes().to_vec())
        }
    }
}
