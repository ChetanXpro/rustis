# Rust Redis-like Server

This project is a simple implementation of a Redis-like server in Rust that uses native capabilities and no external dependencies to parse the Redis Serialization Protocol (RESP).

## Features

- **Simple Key-Value Store**: Supports basic `SET` and `GET` commands.
- **Time-to-Live (TTL)**: Allows keys to have an expiration time.
- **Built-in RESP Parsing**: Uses pure Rust code with no external libraries for RESP parsing.
- **Asynchronous**: Built with Tokio for efficient IO operations and concurrency.

## Supported Commands

- `PING`: Returns "PONG".
- `ECHO <message>`: Echoes back the message provided.
- `SET <key> <value> [TTL]`: Sets a key to the specified value with an optional TTL in seconds.
- `GET <key>`: Retrieves the value of a specified key. Returns an error if the key has expired or does not exist.


## Installation

Clone this repository and build the project using Cargo, Rust's package manager and build tool:

```bash
git clone https://github.com/ChetanXpro/redis
cd redis
cargo build --release
```

### Running the Server

Start the server with:

```bash
cargo run --release
```

The server will listen on 127.0.0.1:8080. You can interact with it using any Redis-compatible client or simple network tools like telnet.


## License
This project is open-sourced under the MIT License. See the LICENSE file for more details.
