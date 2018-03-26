pub mod block;
pub mod transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let block = block::Block::new_genesis();
        let mut chain = Blockchain {
            blocks: [].to_vec(),
        };
        chain.blocks.push(block);
        chain
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks[self.blocks.len() - 1].clone();
        match block::Block::new(data, prev_block.hash.clone()) {
            Ok(v) => {
                self.blocks.push(v);
            }
            Err(_err) => {
                "error block".to_string();
            }
        };
    }
}
