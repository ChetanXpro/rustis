use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream
        .read(buffer.as_mut())
        .expect("Failed to read from stream");

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", request);

    let response = "Yoooo".as_bytes();

    stream.write(response).expect("Failed to write to stream");
}

fn main() {
    print!("Hello")
}
