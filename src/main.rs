mod blockchain;
mod block;

use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("First block"));
    blockchain.add_block(String::from("Second block"));

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
