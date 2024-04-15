use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

use resp::RespHandler;
mod resp;

const MESSAGE_SIZE: usize = 512;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; MESSAGE_SIZE];
    let mut handler = RespHandler::new(stream);

    loop {}
}

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server started at 127.0.0.1:8080");
    listner.incoming().for_each(|stream| match stream {
        Ok(stream) => {
            println!("New client connected");
            std::thread::spawn(|| handle_client(stream));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    });
}
