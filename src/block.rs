
use crate::transaction::Transaction;
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub validator: String,
}
