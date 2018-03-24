use chrono::prelude::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub timestamp: i64,
    pub data: Vec<u8>,
}

impl Block {
    pub fn hash(&mut self) {
        let data = self.data.clone();
        let timestamp = self.timestamp.clone();
        let mut headers = self.prev_hash.clone();
        headers.extend(data.iter().cloned());
        headers.extend(timestamp.to_string().into_bytes().iter().cloned());
        let mut sha = Sha256::new();
        sha.input(&headers);
        self.hash = sha.result()[..].to_vec();
    }
    pub fn new(data: String, prev_hash: Vec<u8>) -> Result<Block, &'static str> {
        let mut block = Block {
            prev_hash: prev_hash,
            hash: [].to_vec(),
            timestamp: Utc::now().timestamp(),
            data: data.into_bytes(),
        };
        block.hash();
        Ok(block)
    }
    pub fn new_genesis() -> Block {
        let mut block = Block {
            prev_hash: [].to_vec(),
            hash: [].to_vec(),
            timestamp: Utc::now().timestamp(),
            data: "Genesis Block".to_string().into_bytes(),
        };
        block.hash();
        block
    }
}
