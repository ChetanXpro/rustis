use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use anyhow::Result;
use resp::{RespHandler, RespType};
mod resp;

const MESSAGE_SIZE: usize = 512;

async fn handle_client(stream: TcpStream) {
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

                "GET" => RespType::BulkString("GET".to_string()),
                "SET" => RespType::BulkString("SET".to_string()),
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
        spawn(async move {
            handle_client(stream).await;
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
