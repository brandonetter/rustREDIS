mod handler;
mod parser;
mod store;
mod types;

use handler::handle_connection;
use std::sync::Arc;
use store::RedisStore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let store = Arc::new(RedisStore::new());
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
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
