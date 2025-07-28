mod transaction;
mod block;
mod blockchain;

use crate::block::Block;
use crate::transaction::Transaction;
fn main() {
}


#[cfg(test)]
mod  test{
use super::*;
#[test]
fn tests_block_transaction() {
    println!("Hello, world!");
    let transaction = Transaction{
        receiver: "0x123".into(),
        sender: "0x123".into(),
        amount: 40
    };
    let mut transactions = Vec::new();
    transactions.push(transaction);
    let genesis_block = Block::new(  1, transactions, "12334".into(), "0x1234".into());
    assert_eq!(genesis_block.transactions.len(), 1);
    assert_eq!(genesis_block.hash, genesis_block.calculate_hash().ok().unwrap());

}

}