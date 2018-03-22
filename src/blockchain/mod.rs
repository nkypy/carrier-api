pub mod block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: String) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new(data, prev_block.prev_hash.clone());
        // self.blocks.push(new_block);
    }
}
