pub mod clock;
pub mod net;
pub mod power_supply;
pub mod utils;

pub struct BlockUpdate {
    block_id: usize,
    full_text: String,
}

impl BlockUpdate {
    pub fn new(block_id: usize, full_text: &str) -> BlockUpdate {
        BlockUpdate {
            block_id: block_id,
            full_text: String::from(full_text),
        }
    }

    pub fn block_id(&self) -> usize {
        self.block_id
    }

    pub fn full_text(&self) -> &str {
        &self.full_text
    }
}
