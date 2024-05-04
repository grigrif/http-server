use std::collections::HashMap;
use std::fmt::Error;
use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::TcpListener;

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    http_version: String,
    headers: HashMap<String, String>
}

fn parse_request(string: &str) -> Result<HttpRequest, Error> {
    let mut lines = string.split("\r\n");
    let fe = lines.next().ok_or(Error)?;
    let method = fe.split(" ").nth(0).ok_or(Error)?;
    let path = fe.split(" ").nth(1).ok_or(Error)?;
    let http_version = fe.split(" ").nth(2).ok_or(Error)?;
    println!("za");
    let mut map = HashMap::new();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break
        }
        let (a, b) = l.split_once(" ").ok_or(Error)?;
        map.insert(a.to_string(), b.to_string());
    }
   Ok (HttpRequest {
        method: method.to_string(), path: path.to_string(), http_version: http_version.to_string(), headers: map
    })
}

fn plain_text(str: &str) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r
\r
{}\r", str.len(), str)
}
fn main() {

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
     for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                 println!("accepted new connection");
                let mut rx_bytes = [0u8; 1024];
                _stream.read(&mut rx_bytes).expect("TODO: panic message");
                let string = std::str::from_utf8(&rx_bytes).expect("valid utf8");


                let request = parse_request(string);
                if let Ok(re) = request {
                    dbg!(&re);
                    if re.path.starts_with("/echo/") {
                        let str = &re.path[6..];
                        dbg!(&str);
                        let m = plain_text(str);
                        _stream.write(m.as_bytes()).unwrap();
                    } else {
                    match re.path.as_str() {
                        "/" => {
                            _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("TODO: panic message");

                        }
                        "/user-agent" => {
                            _stream.write(plain_text(re.headers.get("User-Agent:").unwrap()).as_bytes()).expect("TODO: panic message");
                        }
                        _ => {
                            _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").expect("TODO: panic message");
                        }
                    }}
                } else {
                    println!("Parse Error");
                }
            }
            Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
