use crate::block::Block;
use crate::transaction::Transaction;

use std::env::current_exe;
use std::vec;
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Result<Self, String> {
        let genesis_transaction = Transaction {
            sender: "0x0000".into(),
            receiver: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".into(),
            amount: 1000000,
        };

        let genesis_block = Block::new(0, vec![genesis_transaction], "0".into(), "0x0000".into());
        Ok(Blockchain {
            chain: vec![genesis_block],
        })
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        let latest_block = self.chain.last().unwrap();

        let new_block = Block::new(
            latest_block.index + 1,
            transactions,
            latest_block.hash.clone(),
            latest_block.validator.clone(),
        );
        Ok(self.chain.push(new_block))
    }

    pub fn is_valid(&self) -> bool {

          if self.chain.is_empty() {
                return false;
            }
            let genesis_block = &self.chain[0];
            let hashed_genesis = match genesis_block.calculate_hash(){
                Ok(hash) => hash,
                Err(_) => return  false
            };
        if genesis_block.hash != hashed_genesis{
            return  false;
        }
        for i in 1..self.chain.len() {
          
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let block_hash = current.calculate_hash().unwrap();
            if block_hash != current.hash {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}
