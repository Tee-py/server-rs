# 🦀 server-rs

A Simple Multithreaded Webserver written in rust.

This server is not meant for production. It is for learning purposes.

## Running the server
* clone the repository
* run `cargo run` and head over to your browser to type http://localhost:8000 which displays a welcome page for the server.

## Usage


```rust
// Neccessary Imports
use std::net::{TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

// The webserver and webserver config
use server::{Webserver, WebserverConfig};

// A function that handles http requests coming from the server
fn app(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/welcome.html"),
        "GET /styles.css HTTP/1.1" => ("HTTP/1.1 200 OK", "static/styles.css"),
        "GET /styles1.css HTTP/1.1" => ("HTTP/1.1 200 OK", "static/styles1.css"),
        _ => ("HTTP/1.1 404 NOT FOUND", "static/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// Entry point for the webserver
fn main() {

    let config = WebserverConfig::new(100, "127.0.0.1".to_string(), "8000".to_string());

    let server = Webserver::new(config);

    server.start(app)
}
```

### License

This project is licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)