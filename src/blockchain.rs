
use std::vec;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::utils::get_timestamp;
#[derive(Debug)]
pub struct Blockchain{
    pub chain: Vec<Block>
}

impl  Blockchain {

    pub fn new () -> Result<Self, String>{

        let genesis_block = Block{
            index: 0,
            timestamp: get_timestamp().unwrap(),
            previous_hash: "0".into(),
            transactions: vec![],
            hash: "0".into(),
            validator: "0x0000".into()
        };

       Ok( Blockchain{
            chain: vec![genesis_block]
        })
        
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String>{

        let latest_block = self.chain.last().unwrap();

        let new_block =  Block::new(
            latest_block.index + 1, transactions, 
            latest_block.hash.clone(),
            latest_block.validator.clone()
            );
            Ok(self.chain.push(new_block))
            
    }
}
