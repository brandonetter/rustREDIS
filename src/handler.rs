use crate::parser::parse_command;
use crate::store::RedisStore;
use crate::types::RedisGetResult;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
    tenant: Option<String>,
}

pub async fn handle_connection(mut stream: TcpStream, store: Arc<RedisStore>) {
    let mut connection = Connection { tenant: None };

    let mut buf = [0; 4056];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        let request = String::from_utf8_lossy(&buf[..n]);

        let _ = handle_request(&request, &mut stream, store.clone(), &mut connection).await;
        if n == 0 {
            break;
        }
    }
}

async fn handle_request(
    request: &str,
    stream: &mut TcpStream,
    store: Arc<RedisStore>,
    connection: &mut Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_parts = parse_command(request);

    if !command_parts.is_empty() {
        match command_parts[0].as_str().to_uppercase().as_str() {
            "CLIENT" => {
                if command_parts.len() > 2 && command_parts[1].to_uppercase() == "SETNAME" {
                    connection.tenant = Some(command_parts[2].clone());
                    stream.write_all("+OK\r\n".as_bytes()).await?;
                }
            }
            cmd @ ("SET" | "GET" | "APPEND") => {
                match &connection.tenant {
                    Some(tenant) => {
                        // Add tenant prefix to key
                        let key = if command_parts.len() > 1 {
                            format!("{}:{}", tenant, command_parts[1])
                        } else {
                            return Ok(());
                        };

                        match cmd {
                            "SET" => {
                                if command_parts.len() > 2 {
                                    let px = if command_parts.len() > 4 {
                                        Some(command_parts[4].parse::<u64>().unwrap())
                                    } else {
                                        None
                                    };
                                    store.set(key, command_parts[2].clone(), px)?;
                                    stream.write_all("+OK\r\n".as_bytes()).await?;
                                }
                            }
                            "GET" => {
                                let response: String = match store.get(&key) {
                                    RedisGetResult::Value(value) => format!("+{}\r\n", value),
                                    RedisGetResult::None => "+\r\n".to_string(),
                                    RedisGetResult::Expired => "$-1\r\n".to_string(),
                                };
                                stream.write_all(response.as_bytes()).await?;
                            }
                            "APPEND" => {
                                if command_parts.len() > 2 {
                                    let response = match store.append(key, command_parts[2].clone())
                                    {
                                        Ok(_) => "+OK\r\n".to_string(),
                                        Err(e) => format!("-ERR {}\r\n", e),
                                    };
                                    stream.write_all(response.as_bytes()).await?;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        stream
                            .write_all(
                                "-ERR Tenant name required (use CLIENT SETNAME)\r\n".as_bytes(),
                            )
                            .await?;
                    }
                }
            }
            "PING" | "HEALTH" => {
                stream.write_all("+PONG\r\n".as_bytes()).await?;
            }
            "ECHO" => {
                if command_parts.len() > 1 {
                    stream
                        .write_all(format!("+{}\r\n", command_parts[1]).as_bytes())
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
