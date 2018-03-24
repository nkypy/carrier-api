extern crate byteorder;
extern crate chrono;
extern crate sha2;

mod blockchain;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match blockchain::block::Block::new("transfer".to_string(), [1u8].to_vec()) {
        Err(err) => println!("{:?}", err),
        Ok(w) => {
            println!("{:?}", w);
            let data = w.data.clone();
            let s = match String::from_utf8(data) {
                Ok(v) => v,
                Err(err) => "invalid string".to_string(),
            };
            println!("block data {}", s);
        }
    };
}
