use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

const MESSAGE_SIZE: usize = 1024;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; MESSAGE_SIZE];

    stream
        .read(buffer.as_mut())
        .expect("Failed to read from stream");

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", request);

    let response = b"+PONG\r\n";

    stream
        .write_all(response)
        .expect("Failed to write to stream");
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
