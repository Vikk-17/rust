use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{sha256, Digest};
use serde::{Deserialize, Serialize};

// Block: a struct that represents a block in a Blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {}
