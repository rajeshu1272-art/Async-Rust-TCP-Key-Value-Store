use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Rust Key Value Store running on 127.0.0.1:6379");

    let store = Arc::new(Mutex::new(HashMap::new()));

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let store = store.clone();
                tokio::spawn(async move {
                    handle_client(socket, store).await;
                });
            }
            Err(e) => eprintln!("failed to accept socket; error = {:?}", e),
        }
    }
}

async fn handle_client(socket: TcpStream, store: Arc<Mutex<HashMap<String, String>>>) {
    let mut stream = BufReader::new(socket);
    let mut line = String::new();

    loop {
        line.clear();
        let _n = match stream.read_line(&mut line).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let input = line.trim().to_string();
        let parts: Vec<&str> = input.split_whitespace().collect();

        let mut response = String::new();

        if parts.len() >= 2 && parts[0] == "GET" {
            let key = parts[1];
            match store.lock() {
                Ok(map) => {
                    response = match map.get(key) {
                        Some(val) => format!("{}\n", val),
                        None => "NULL\n".to_string(),
                    };
                }
                Err(_) => response = "ERROR: Internal Server Error\n".to_string(),
            }
        } 
        else if parts.len() >= 3 && parts[0] == "SET" {
            let key = parts[1].to_string();
            let value = parts[2].to_string();
            match store.lock() {
                Ok(mut map) => {
                    map.insert(key, value);
                    response = "OK\n".to_string();
                }
                Err(_) => response = "ERROR: Internal Server Error\n".to_string(),
            }
        } 
        else {
            response = "ERROR: Use SET key value or GET key\n".to_string();
        }

        let _ = stream.write_all(response.as_bytes()).await;
    }
}
