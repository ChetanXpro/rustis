use anyhow::{anyhow, Result};
use bytes::BytesMut;
use std::fmt;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const CRLF: &str = "\r\n";

#[derive(Clone, Debug)]
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    Array(Vec<RespType>),
    BulkString(String),
}

impl fmt::Display for RespType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RespType::SimpleString(s) => write!(f, "{}", s),
            RespType::Error(s) => write!(f, "{}", s),
            RespType::Integer(i) => write!(f, "{}", i),
            RespType::Array(arr) => {
                let items = arr
                    .iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", items)
            }
            RespType::BulkString(s) => write!(f, "{}", s),
        }
    }
}

impl RespType {
    pub fn serialize(self) -> String {
        match self {
            RespType::SimpleString(s) => format!("+{}\r\n", s),
            RespType::Error(e) => format!("-{}\r\n", e),
            RespType::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            RespType::Integer(i) => format!(":{}\r\n", i),
            RespType::Array(elements) => {
                let serialized_elements = elements
                    .clone() // Clone the vector
                    .into_iter()
                    .map(|element| element.serialize())
                    .collect::<Vec<String>>()
                    .join("");
                format!("*{}\r\n{}", elements.len(), serialized_elements)
            }
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

    pub async fn read_value(&mut self) -> Result<Option<RespType>> {
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

        if bytes_read == 0 {
            return Ok(None);
        }

        let (v, _) = parse_message(self.buffer.split())?;

        Ok(Some(v))
    }

    pub async fn write_value(&mut self, value: RespType) -> Result<()> {
        self.stream.write(value.serialize().as_bytes()).await?;

        Ok(())
    }
}

fn parse_message(buffer: BytesMut) -> Result<(RespType, usize)> {
    match buffer[0] as char {
        '+' => parse_simple_string(buffer),
        '-' => parse_error(buffer),
        '*' => parse_array(buffer),
        // ':' => parse_integer(buffer),
        '$' => parse_bulk_string(buffer),
        _ => Err(anyhow::anyhow!("Not a known message type {:?}", buffer)),
    }
}

// fn parse_integer(buffer: BytesMut) -> Result<(RespType, unsize)> {

// }

fn parse_error(buffer: BytesMut) -> Result<(RespType, usize)> {
    let slice = &buffer[1..];
    if let Some((line, length)) = read_until_crlf(slice) {
        Ok((
            RespType::Error(String::from_utf8_lossy(line).to_string()),
            length,
        ))
    } else {
        Err(anyhow!("Incomplete message"))
    }
}

fn parse_simple_string(buffer: BytesMut) -> Result<(RespType, usize)> {
    if buffer.is_empty() {
        return Err(anyhow::anyhow!("Buffer is empty"));
    }

    let slice = &buffer[1..];

    if let Some((line, length)) = read_until_crlf(slice) {
        Ok((
            RespType::SimpleString(String::from_utf8_lossy(line).to_string()),
            length,
        ))
    } else {
        Err(anyhow::anyhow!("Incomplete message"))
    }
}

// fn parse_error(buffer: BytesMut) -> Result<(RespType, usize)> {}

fn parse_int(buffer: &[u8]) -> Result<i64> {
    Ok(String::from_utf8(buffer.to_vec())
        .map_err(|e| anyhow!(e))?
        .parse::<i64>()?)
}

// $5\r\nhello\r\n
fn parse_bulk_string(buffer: BytesMut) -> Result<(RespType, usize)> {
    let (buld_string_length, bytes_consumed) =
        if let Some((line, length)) = read_until_crlf(&&buffer[1..]) {
            let buld_string_length = parse_int(line)?;

            (buld_string_length, length + 1)
        } else {
            return Err(anyhow::anyhow!("Invalid bulk string format {:?}", buffer));
        };

    let bulk_string_end = bytes_consumed + buld_string_length as usize;
    let total_parsed = bulk_string_end + 2;
    Ok((
        RespType::BulkString(
            String::from_utf8(buffer[bytes_consumed..bulk_string_end].to_vec())?.into(),
        ),
        total_parsed,
    ))
}

// *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
fn parse_array(buffer: BytesMut) -> Result<(RespType, usize)> {
    let (array_length, mut bytes_consumed) =
        if let Some((line, length)) = read_until_crlf(&&buffer[1..]) {
            let array_length = parse_int(line)?;

            (array_length, length + 1)
        } else {
            return Err(anyhow::anyhow!("Incomplete message"));
        };

    let mut array_items = vec![];
    for _ in 0..array_length {
        let (array_item, len) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))?;

        array_items.push(array_item);
        bytes_consumed += len;
    }

    Ok((RespType::Array(array_items), bytes_consumed))
}

fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == b'\r' && buffer.get(i) == Some(&b'\n') {
            return Some((&buffer[..(i - 1)], i + 1));
        }
    }

    None
}
