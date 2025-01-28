#[derive(Clone)]
pub struct Transaction {

}

#[derive(Clone)]
pub struct DataBlock {
    pub timestamp: u128,                 //block creation time
    pub pre_block_hash: String,         //the hash value of the previous block
    pub transactions: Vec<Transaction>, //list of transactions included into block    
    pub height: usize,                  //block's position in the blockchain. Number of blocks before the current
}