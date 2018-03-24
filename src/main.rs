extern crate byteorder;
extern crate chrono;
extern crate sha2;

mod blockchain;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let mut bc = blockchain::Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan".to_string());
    bc.add_block("Send 2 more BTC to Ivan".to_string());
    println!("Blockchain: {:?}", bc)
}
