mod blockchain;
mod block;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    // Test cases
    match blockchain.add_block("Carlos_id_vote".to_string(), "election1".to_string(), "option1".to_string()) {
        Ok(_) => println!("Carlos_id_vote voted successfully in election1"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    match blockchain.add_block("Carlos_id_vote".to_string(), "election1".to_string(), "option2".to_string()) {
        Ok(_) => println!("Carlos_id_vote voted successfully in election1 again"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    match blockchain.add_block("Maria_id_vote".to_string(), "election1".to_string(), "option1".to_string()) {
        Ok(_) => println!("Maria_id_vote voted successfully in election1"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    // New test cases for voting in different elections
    match blockchain.add_block("Carlos_id_vote".to_string(), "election2".to_string(), "option1".to_string()) {
        Ok(_) => println!("Carlos_id_vote voted successfully in election2"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    match blockchain.add_block("Carlos_id_vote".to_string(), "election2".to_string(), "option1".to_string()) {
        Ok(_) => println!("Carlos_id_vote voted successfully in election2 again"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    match blockchain.add_block("Maria_id_vote".to_string(), "election2".to_string(), "option2".to_string()) {
        Ok(_) => println!("Maria_id_vote voted successfully in election2"),
        Err(e) => println!("Failed to vote: {}", e),
    }

    println!("{:#?}", blockchain);
}
