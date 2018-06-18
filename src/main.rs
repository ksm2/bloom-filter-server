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
use std::option::NoneError;

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
        let mut buf = [0u8; 4096];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read < 3 {
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

trait IsWhitespace {
    fn is_whitespace(&self) -> bool;
}

impl IsWhitespace for u8 {
    fn is_whitespace(&self) -> bool {
        *self == b'\t' || *self == b' ' || *self == 13 || *self == 10
    }
}


trait SplitWhitespace {
    fn split_whitespace(&self) -> Result<Vec<&[u8]>, std::option::NoneError>;
}

impl SplitWhitespace for [u8] {
    /// Splits the byte array into chunks on whitespace.
    fn split_whitespace(&self) -> Result<Vec<&[u8]>, std::option::NoneError> {
        let mut start = 0;
        let mut vec = Vec::new();
        let mut i = 0;
        while i < self.len() {
            if self[i] == 0 {
                return Ok(vec);
            }

            if self[i].is_whitespace() {
                vec.push(self.get(start..i)?);
                i += 1;
                while i < self.len() - 1 && self[i + 1].is_whitespace() {
                    i += 1;
                }
                start = i;
            }

            i += 1;
        }
        Ok(vec)
    }
}

/// Handles an incoming message.
fn handle_message(bf: &mut BloomFilter, message: [u8; 4096]) -> Result<Vec<u8>, NoneError> {
    let command: &[u8] = message.get(0..3)?;
    let rest: &[u8] = message.get(4..)?;
    let tokens = rest.split_whitespace()?;

    match command {
        b"ADD" | b"add" => {
            bf.add(tokens);
            Ok(b"OK.\n".to_vec())
        }
        b"RMV" | b"rmv" => {
            for token in tokens {
                bf.remove(token);
                println!("Removed '{}'", String::from_utf8_lossy(token).to_string());
            }
            Ok(b"OK.\n".to_vec())
        }
        b"HAS" | b"has" => {
            let token = tokens.iter().next()?;
            let is_contained = bf.has(token);
            println!("Check if '{}' is contained: {}", String::from_utf8_lossy(token).to_string(), is_contained);
            if is_contained {
                Ok(b"Yes.\n".to_vec())
            } else {
                Ok(b"No.\n".to_vec())
            }
        }
        b"CNT" | b"cnt" => {
            let token = tokens.iter().next()?;
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
