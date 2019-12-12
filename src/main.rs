#![feature(test)]

extern crate bit_vec;
extern crate murmur3;
extern crate rayon;
extern crate test;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use bloom_filter::BloomFilter;

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
            Ok(stream) => {
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
            break;
        }

        let (child_tx, parent_rx) = channel::<Vec<u8>>();
        parent_tx.send((buf, child_tx)).unwrap();
        let to_send = parent_rx.recv().unwrap();
        stream.write(to_send.as_slice()).unwrap();
    }

    println!("Closed connection to client #{}", id);
}

/// Handles messages from incoming clients and holds the Bloom filter
fn handle_server(rx: Receiver<([u8; 4096], Sender<Vec<u8>>)>) {
    let mut bf = BloomFilter::new();

    for (message, tx) in rx.iter() {
        let mut prefix = "ERROR. ".to_owned();
        match handle_message(&mut bf, message) {
            Ok(to_send) => tx.send(to_send).unwrap(),
            Err(err) => {
                prefix.push_str(err);
                prefix.push('\n');
                tx.send(prefix.into_bytes()).unwrap()
            },
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
    fn split_whitespace(&self) -> Result<Vec<&[u8]>, &'static str>;
}

impl SplitWhitespace for [u8] {
    /// Splits the byte array into chunks on whitespace.
    fn split_whitespace(&self) -> Result<Vec<&[u8]>, &'static str> {
        let mut start = 0;
        let mut vec = Vec::new();
        let mut i = 0;
        while i < self.len() {
            let eof = self[i] == 0;
            if eof || self[i].is_whitespace() {
                if i > start {
                    match self.get(start..i) {
                        Some(element) => vec.push(element),
                        None => return Err("Could not get next element."),
                    };
                }

                if eof {
                    return Ok(vec);
                } else {
                    start = i + 1;
                }
            }

            i += 1;
        }
        Ok(vec)
    }
}

/// Handles an incoming message.
fn handle_message(bf: &mut BloomFilter, message: [u8; 4096]) -> Result<Vec<u8>, &'static str> {
    let command: &[u8] = match message.get(0..3) {
        Some(x) => x,
        None => return Err("Could not get command."),
    };

    let rest: &[u8] = match message.get(4..) {
        Some(x) => x,
        None => return Err("Could not get argument."),
    };

    let tokens = rest.split_whitespace()?;

    match command {
        b"ADD" | b"add" => {
            if tokens.len() == 0 {
                return Err("Specify at least one element.");
            }
            println!("Added {} element(s)", tokens.len());
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
            let token = match tokens.iter().next() {
                Some(t) => t,
                None => return Err("Could not get token"),
            };
            let is_contained = bf.has(token);
            println!("Check if '{}' is contained: {}", String::from_utf8_lossy(token).to_string(), is_contained);
            if is_contained {
                Ok(b"Yes.\n".to_vec())
            } else {
                Ok(b"No.\n".to_vec())
            }
        }
        b"CNT" | b"cnt" => {
            let token = match tokens.iter().next() {
                Some(t) => t,
                None => return Err("Could not get token"),
            };
            let count = bf.count(token);
            Ok(format!("{}.\n", count).into_bytes())
        }
        b"BIN" | b"bin" => {
            Ok(bf.to_bytes())
        }
        _ => {
            println!("Error with incoming message.");
            Ok(format!("ERROR. Invalid command {}.\n", String::from_utf8_lossy(command).trim_end()).as_bytes().to_vec())
        }
    }
}
