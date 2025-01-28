use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::data_block::{DataBlock, Transaction};
use crate::proof_of_work::proof_of_work;


pub struct Block {
    block_data: DataBlock,
    nonce: i64,                     //number only used once    
    hash: String,                   //hash value of the current block, block's unique identifier
}

impl Block {    
    pub fn new(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Self {           
        let block_data = DataBlock {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            pre_block_hash,
            height,
            transactions: transactions.to_vec()
        };    
        let (hash, nonce) = proof_of_work(&block_data);
        Self {block_data, hash, nonce}
    }
}