mod block;
mod blockchain;
mod transaction;
mod utils;
mod stake;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
fn main() {

    let transaction = Transaction {
        receiver: "0x123".into(),
        sender: "0x123".into(),
        amount: 40,
    };

    let transaction1 = Transaction {
        receiver: "0x123".into(),
        sender: "0x123".into(),
        amount: 100,
    };
    let transactions = vec![transaction];
    let mut blockchain = Blockchain::new().unwrap();
    blockchain.stake("0xA04889f942DE04232689dE59fc6221cAabA9A65B".to_string(), 100).ok();
    blockchain.stake("0xBDB14399e2a300565DF4515e810a1355832446E4".to_string(), 300).ok();
    blockchain.add_block(transactions).ok();
    println!("{:#?}", blockchain);
    // blockchain.chain[0].index = 10;
    println!("{:#?}", blockchain);
    println!("{:?}", blockchain.is_valid());
    blockchain.stake("0x123".into(), 45);
    println!("{:#?}", blockchain);
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    #[test]
    fn tests_genesis_block() {
        let transaction = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 40,
        };
        let transactions = vec![transaction];

        match Blockchain::new() {
            Ok(mut blockchain) => {
                assert_eq!(blockchain.chain.len(), 1);
                assert_eq!(blockchain.chain[0].transactions.len(), 1);
                blockchain.add_block(transactions).ok().unwrap();
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[test]
    fn tests_add_block() {
        println!("Hello, world!");
        let transaction = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 40,
        };
        let transactions = vec![transaction];

        match Blockchain::new() {
            Ok(mut blockchain) => {
                blockchain.add_block(transactions).ok().unwrap();
                assert_eq!(blockchain.chain.len(), 2);
                assert_eq!(blockchain.chain[1].transactions.len(), 1);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_valid_block() {
        let transaction = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 40,
        };

        let transaction1 = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 100,
        };
        let transactions = vec![transaction, transaction1];

        match Blockchain::new() {
            Ok(mut blockchain) => {
                assert_eq!(blockchain.chain.len(), 1);
                blockchain.add_block(transactions).ok().unwrap();
                assert_eq!(blockchain.chain[1].transactions.len(), 2);
                assert_eq!(blockchain.is_valid(), true);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_invalid_block() {
        let transaction = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 40,
        };

        let transaction1 = Transaction {
            receiver: "0x123".into(),
            sender: "0x123".into(),
            amount: 100,
        };
        let transactions = vec![transaction, transaction1];

        match Blockchain::new() {
            Ok(mut blockchain) => {
                assert_eq!(blockchain.chain.len(), 1);
                blockchain.add_block(transactions).ok().unwrap();
                assert_eq!(blockchain.chain[1].transactions.len(), 2);
                blockchain.chain[1].index = 10;
                assert_eq!(blockchain.is_valid(), false);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
