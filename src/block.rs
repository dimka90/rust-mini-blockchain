use crate::transaction::{Transaction};
use sha2::{Digest, Sha256};
use crate::utils::utils::get_timestamp;
#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        validator: String,
    ) -> Self {
        let current_time = get_timestamp().unwrap();

        let mut block = Block {
            index,
            timestamp: current_time,
            transactions,
            previous_hash,
            hash: String::new(),
            validator,
        };
        block.hash = block.calculate_hash().ok().unwrap();
        block
    }

    pub fn calculate_hash(&self) -> Result<String, String> {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.validator
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        Ok(result.iter().map(|byte| format!("{:02x}", byte)).collect())
    }
}
