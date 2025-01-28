use crate::data_block::DataBlock;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;
use std::ops::ShlAssign;

const TARGET_BITS: i32 = 8; 

pub fn proof_of_work(block_data: &DataBlock) -> (String, i64) {
    let target: BigInt = BigInt::from(1) << (256 - TARGET_BITS);//BigInt::from(1).shl_assign(rhs);    
    (String::new(), 0)
}