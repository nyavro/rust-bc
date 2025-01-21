pub struct Block {
    timestamp: i64,                 //block creation time
    pre_block_hash: String,         //the hash value of the previous block
    hash: String,                   //hash value of the current block, block's unique identifier
    transactions: Vec<Transaction>, //list of transactions included into block
    nonce: i64,                     //number only used once
    height: usize,                  //block's position in the blockchain. Number of blocks before the current
}

pub fn new_block(pre_block_hash: String, transactions: &[Transaction],
    height: usize) -> Block {
    
}