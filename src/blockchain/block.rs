use chrono::prelude::Utc;

#[derive(Debug)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub timestamp: i64,
    pub data: Vec<u8>,
}

impl Block {
    pub fn hash(&mut self) {
        let data = self.data.clone();
        let mut headers = self.prev_hash.clone();
        headers.extend(data.iter().cloned());
        // let mut sha = Sha256::default();
        // sha.input(data);
        // self.hash = sha.result_bytes().to_vec();
        self.hash = headers.to_vec();
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
        // Err("测试错误")
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
