use chrono::prelude::*;
use super::block::Block;

type Blocks = Vec<Block>;

// Blockchain: A struct that represents the blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {}

