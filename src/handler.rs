use crate::parser::parse_command;
use crate::store::RedisStore;
use crate::types::RedisGetResult;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_connection(mut stream: TcpStream, store: Arc<RedisStore>) {
    let mut buf = [0; 4056];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        let request = String::from_utf8_lossy(&buf[..n]);
        let _ = handle_request(&request, &mut stream, store.clone()).await;
        if n == 0 {
            break;
        }
    }
}

async fn handle_request(
    request: &str,
    stream: &mut TcpStream,
    store: Arc<RedisStore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_parts = parse_command(request);

    if !command_parts.is_empty() {
        match command_parts[0].as_str().to_uppercase().as_str() {
            "ECHO" => {
                if command_parts.len() > 1 {
                    let response = format!("+{}\r\n", command_parts[1]);
                    stream.write_all(response.as_bytes()).await?;
                }
            }
            "HEALTH" | "PING" => {
                stream.write_all("+PONG\r\n".as_bytes()).await?;
            }
            "SET" => {
                if command_parts.len() > 2 {
                    let response = format!("+OK\r\n");
                    let px = if command_parts.len() > 4 {
                        Some(command_parts[4].parse::<u64>().unwrap())
                    } else {
                        None
                    };
                    store.set(command_parts[1].clone(), command_parts[2].clone(), px)?;
                    stream.write_all(response.as_bytes()).await?;
                }
            }
            "GET" => {
                if command_parts.len() > 1 {
                    let response: String = match store.get(&command_parts[1]) {
                        RedisGetResult::Value(value) => format!("+{}\r\n", value),
                        RedisGetResult::None => "+\r\n".to_string(),
                        RedisGetResult::Expired => "$-1\r\n".to_string(),
                    };
                    stream.write_all(response.as_bytes()).await?;
                }
            }
            "APPEND" => {
                if command_parts.len() > 2 {
                    let response =
                        match store.append(command_parts[1].clone(), command_parts[2].clone()) {
                            Ok(_) => "+OK\r\n".to_string(),
                            Err(e) => format!("-ERR {}\r\n", e).to_string(),
                        };
                    stream.write_all(response.as_bytes()).await?;
                } else {
                    stream
                        .write_all(
                            "-ERR wrong number of arguments for 'append' command\r\n".as_bytes(),
                        )
                        .await?;
                }
            }
            "INFO" => {
                let response = format!("+{}\r\n", "redis_version:0.0.1");
                stream.write_all(response.as_bytes()).await?;
            }
            _ => {
                println!("Unknown command: {:?}", command_parts);
            }
        }
    }
    Ok(())
}
