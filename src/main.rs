use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

const MESSAGE_SIZE: usize = 512;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; MESSAGE_SIZE];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer).trim().to_string();
                println!("Request: {}", request);

                match request.as_str() {
                    "ping" => {
                        let response = b"+PONG\r\n";
                        stream
                            .write_all(response)
                            .expect("Failed to write to stream");
                    }
                    "echo" => {
                        const ECHO_PREFIX: &str = "+";
                        let response = b"+{}\r\n";
                        stream
                            .write_all(response)
                            .expect("Failed to write to stream");
                    }
                    _ => {
                        let response = b"+OK\r\n";
                        stream
                            .write_all(response)
                            .expect("Failed to write to stream");
                    }
                }

                buffer.fill(0);
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
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
