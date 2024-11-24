mod handler;
mod parser;
mod search_parser;
mod store;
mod types;

use handler::handle_connection;
use std::env;
use std::sync::Arc;
use store::RedisStore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "6379".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("⚡ Starting server on {}", addr);

    let store = Arc::new(RedisStore::new());

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => {
            println!("✅ Successfully bound to {}", addr);
            l
        }
        Err(e) => {
            eprintln!("❌ Failed to bind: {}", e);
            return Err(e.into());
        }
    };

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("📡 Connection from: {}", addr);
                let store_clone = Arc::clone(&store);
                tokio::spawn(async move {
                    handle_connection(stream, store_clone).await;
                });
            }
            Err(e) => eprintln!("❌ Accept error: {}", e),
        }
    }
}
