#![feature(extern_prelude)]

extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate sha2;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod blockchain;

use rustc_serialize::hex::ToHex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}\n", args);
    let mut bc = blockchain::Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan".to_string());
    bc.add_block("Pay 0.1 BTC for a cup of coffee".to_string());
    bc.add_block("赠送 1 比特币给 Ivan".to_string());
    println!("Blockchain: {:?}\n", bc);
    for v in bc.blocks {
        let block = v.clone();
        println!(
            "PrevHash: {}\nData: {:?}\nHash: {}\n",
            block.prev_hash.clone().to_hex(),
            String::from_utf8(block.data.clone()),
            block.hash.clone().to_hex()
        );
    }
    let tx = blockchain::Blockchain::find_transaction("12".to_string());
    println!("Transaction {:?}", tx);
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
