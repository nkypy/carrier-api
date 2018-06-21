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
#[macro_use]
extern crate lazy_static;

mod blockchain;
mod rpc;

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
    println!("Transaction {:?}\n", tx);
    let x = 5 + /* 90 + */ 5;
    let y = format!("Is `x` 10 or 100? x = {}", x);
    println!("{y}\n", y = y);
    let rpc_data = rpc::error::Error::new(100001, "测试 new");
    let serialized = serde_json::to_string(&rpc_data).unwrap();
    let serialized2 = serde_json::to_string(&rpc::error::ERROR_TOKEN_VERIFY_FAILED).unwrap();
    println!("Serialized {}, Serialized2 {}\n", serialized, serialized2);
    let dat = "{\"error_code\":100009,\"error_message\":\"测试正确解析\"}";
    let ha: rpc::error::Error = serde_json::from_str(dat).unwrap();
    println!("json struct {:?}", ha);
}
