use bytes::BytesMut;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

const CRLF: &str = "\r\n";
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
}

impl RespType {
    pub fn serialize(self) -> String {
        match &self {
            Value::SimpleString(s) => format!("+{}\r\n", s),
            Value::Error(e) => format!("-{}\r\n", e),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), String::from_utf8_lossy(s)),
            Value::Integer(i) => format!(":{}\r\n", i),
            _ => unimplemented!(),
        }
    }
}

pub struct RespHandler {
    stream: TcpStream,
    buffer: BytesMut,
}

impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        RespHandler {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn read_value(&self) -> Result<Value, std::io::Error> {}

    pub async fn write_value(&mut self, value: Value) -> Result<(), std::io::Error> {}
}

fn parse_message(buffer: BytesMut) -> Result<RespType, usize> {
    match buffer[0] as char {
        '+' => parse_simple_string(buffer),
        '-' => parse_error(buffer),
        '*' => parse_array(buffer),
        ':' => parse_integer(buffer),
        '$' => parse_bulk_string(buffer),
        _ => Err("Invalid message type"),
    }
}

fn parse_simple_string(buffer: BytesMut) -> Result<(RespType, unsize)> {
    if let Some(line, length) = read_until_crlf([1..]) {
        Ok((
            RespType::SimpleString(String::from_utf8_lossy(line).to_string()),
            length,
        ))
    } else {
        Err("Incomplete message")
    }
}

fn parse_error(buffer: BytesMut) -> Result<(RespType, unsize)> {}
fn parse_int(buffer: BytesMut) -> Result<i64> {
    String::from_utf8(buffer.to_vec())?.parse::<i64>()
}

// $5\r\nhello\r\n
fn parse_bulk_string(buffer: BytesMut) -> Result<(RespType, unsize)> {
    if let Some(line, length) = read_until_crlf([1..]) {
        Ok((
            RespType::SimpleString(String::from_utf8_lossy(line).to_string()),
            length,
        ))
    } else {
        Err("Incomplete message")
    }
}

// *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
fn parse_array(buffer: BytesMut) -> Result<(RespType, unsize)> {
    let (array_length, mut bytes_consumed) = if let Some(line, length) = read_until_crlf([1..]) {
        let array_length = parse_int(line)?;

        (array_length)
    } else {
        Err("Incomplete message")
    };

    let mut array_items = vec![];
    for _ in 0..array_length {
        let (array_item, len) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))?;

        array_items.push(array_item);
        bytes_consumed += len;
    }

    Ok((RespType::Array(array_items), bytes_consumed))
}

fn read_until_crlf(buffer: &[u8]) -> Option<usize> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == b'\r' && buffer.get(i) == Some(&b'\n') {
            return Some((&buffer[..(i - 1)], i + 1));
        }
    }

    None
}
