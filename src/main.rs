mod handler;
mod parser;
mod store;
mod types;

use handler::handle_connection;
use std::env;
use std::sync::Arc;
use store::RedisStore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "6379".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    let store = Arc::new(RedisStore::new());
    println!("Listening on {}", addr);
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Accepted connection");
                let store_clone = store.clone();
                tokio::spawn(async move {
                    handle_connection(stream, store_clone).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
