use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::HashMap;
use std::vec;
use rand::Rng;
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub validators: HashMap<String, u64>

}

impl Blockchain {
    pub fn new() -> Result<Self, String> {
        // Hardcorded data {Genesis block}
        let genesis_transaction = Transaction {
            sender: "0x12B5c5D3FDac52C89e8C450c7a78df515FfFBb77".into(),
            receiver: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".into(),
            amount: 1000000,
        };

        let genesis_block = Block::new(0, vec![genesis_transaction], "0".into(), "0x0000".into());
        Ok(Blockchain {
            chain: vec![genesis_block],
            validators: HashMap::new(),
        })
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        let latest_block = self.chain.last().unwrap();

        let validator = match  self.select_validator() {
            Some(address) => address,
            None=>{
                return Err("Validator Error".into());
            }
        };
        let new_block = Block::new(
            latest_block.index + 1,
            transactions,
            latest_block.hash.clone(),
            validator
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

    pub fn stake(&mut self, address: String, amount: u64) -> Result<(), String>{
    if amount < 32 {
        return  Err("Amount to stake must be greater than or equal to 32 eth".into());
    };
    // Todo
    // match self.validators.get( &address) {
    //     Some(stake) =>{
    //       stake += amount;
    //     },
    //     None => 
    // }
    // Ok(())
    // }
    self.validators.insert(address, amount);
    Ok(())
}
     pub fn select_validator(&self) ->Option<String>{
        let total_stake: u64 = self.validators.values().sum();
        if total_stake == 0 {return None };

        let mut  rng =rand::thread_rng();
        let mut  target = rng.gen_range(0..total_stake);

        for (address, stake) in &self.validators{
            if target < *stake {
               return   Some(address.clone())
            };
            target -= stake;
        };
        None
     }
}
