mod blockchain;
mod block;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Genesis Block".to_string());
    blockchain.add_block("Second Block".to_string());

    println!("{:#?}", blockchain);
}
