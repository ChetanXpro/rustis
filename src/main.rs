use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use anyhow::Result;
use resp::{RespHandler, RespType};
mod resp;
mod state;
use state::ServerState;
const MESSAGE_SIZE: usize = 512;

async fn handle_client(stream: TcpStream, state: Arc<ServerState>) {
    let mut buffer = [0; MESSAGE_SIZE];
    let mut handler = RespHandler::new(stream);

    loop {
        let value = handler.read_value().await.unwrap();

        let response = if let Some(value) = value {
            let (command, args) = get_command(value).unwrap();

            println!("Command: {}", command);
            println!("Args: {:?}", args);
            match command.as_str() {
                "ping" => RespType::SimpleString("PONG".to_string()),

                "echo" => args.first().unwrap().clone(),

                "get" => {
                    if let Some(RespType::BulkString(key)) = args.first() {
                        if let Some(value) = state.get(key.clone()).await {
                            RespType::BulkString(value)
                        } else {
                            RespType::Error("Key not found".to_string())
                        }
                    } else {
                        RespType::Error("Invalid command".to_string())
                    }
                }

                "set" => {
                    if args.len() > 1 {
                        if let (RespType::BulkString(key), RespType::BulkString(value)) =
                            (&args[0], &args[1])
                        {
                            state.set(key.to_string(), value.to_string()).await;
                            RespType::SimpleString("OK".to_string())
                        } else {
                            RespType::Error("Invalid command".to_string())
                        }
                    } else {
                        RespType::Error("Invalid command".to_string())
                    }
                }
                _ => RespType::Error("Invalid command".to_string()),
            }
        } else {
            break;
        };

        handler.write_value(response).await.unwrap();
    }
}
#[tokio::main]
async fn main() {
    let state = ServerState::new();
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");
    println!("Server started at 127.0.0.1:8080");

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        println!("New client connected");
        let state_clone = state.clone();
        spawn(async move {
            handle_client(stream, state_clone).await;
        });
    }
}

fn get_command(value: RespType) -> Result<(String, Vec<RespType>)> {
    match value {
        RespType::Array(mut values) => Ok((
            unpack_bulk_string(values.first().unwrap().clone())?,
            values.into_iter().skip(1).collect(),
        )),
        _ => Err(anyhow::anyhow!("Invalid command")),
    }
}

fn unpack_bulk_string(value: RespType) -> Result<String> {
    match value {
        RespType::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Invalid command")),
    }
}
