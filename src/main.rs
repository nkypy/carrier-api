extern crate chrono;
extern crate sha2;
extern crate rustc_serialize;

mod blockchain;

use std::env;
use rustc_serialize::hex::ToHex;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let mut bc = blockchain::Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan".to_string());
    bc.add_block("Send 2 more BTC to Ivan".to_string());
    println!("Blockchain: {:?}", bc);
    let hashed = bc.blocks[0].clone().hash.clone();
    println!("hash is {}", hashed.to_hex());
}
