mod blockchain;
mod block;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serde_json::Value;
use blockchain::Blockchain;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

fn main() {
    
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let listener = TcpListener::bind("0.0.0.0:3500").unwrap();

    println!("Rodando em 3500");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let blockchain = Arc::clone(&blockchain);
                std::thread::spawn(move || {
                    handle_client(stream, blockchain);
                });
            }
            Err(e) => {
                println!("Erro: {}", e);
            }
        }
    }

    /* // Test cases
    let mut blockchain = blockchain.lock().unwrap();
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

    println!("{:#?}", blockchain); */

}

fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /vote") => handle_post_request(r, blockchain),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}

fn handle_post_request(request: &str, blockchain: Arc<Mutex<Blockchain>>) -> (String, String) {
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
    let json: Result<Value, _> = serde_json::from_str(body);

    match json {
        Ok(data) => {
            let user_id = data["user_id"].as_str().unwrap_or("");
            let election = data["election"].as_str().unwrap_or("");
            let option = data["option"].as_str().unwrap_or("");

            let mut blockchain = blockchain.lock().unwrap();
            match blockchain.add_block(user_id.to_string(), election.to_string(), option.to_string()) {
                Ok(_) => {
                    println!("Blockchain: {:#?}", blockchain);
                    return (OK_RESPONSE.to_string(), "Voto registrado com sucesso".to_string())},
                Err(_e) => {
                    return (INTERNAL_SERVER_ERROR.to_string(), "Voter has already voted in this election".to_string()) 
                }
            }
            
        }
        Err(_) => {
            return (NOT_FOUND.to_string(), "Dados inválidos".to_string());
        }
    }
}


/* //handle_get_request function
fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_all_request function
fn handle_get_all_request(request: &str) -> (String, String) {
    println!("aiii");
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
} */