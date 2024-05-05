mod httprequest;
mod response;
mod router;

use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::Shutdown::Both;
// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::thread;
use tokio::io::AsyncReadExt;
use std::borrow::Borrow;
use crate::httprequest::HttpRequest;
use crate::response::Response;
use crate::router::{build_route, Route, Router};


fn plain_text(str: &str) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r
\r
{}\r", str.len(), str)
}
fn not_found() -> &'static [u8; 26] {
    b"HTTP/1.1 404 Not Found\r\n\r\n"
}
//
fn main() {
    let one = || 1;

    // Uncomment this block to pass the first stage
    //
    let args: Vec<String> = std::env::args().collect();
    let mut directory: String = String::from("test");
    for i in 0..args.len() {
        if args.get(i).unwrap() == "--directory" {
            directory = args.get(i+1).unwrap().clone();
        }
    }
    println!("Serving files from directory: {}", &directory);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let mut router = build_route();

     for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let DIRECTORY = directory.clone();
                let router = router.clone();

                thread::spawn(move || {
                 println!("accepted new connection");
                let mut rx_bytes = [0u8; 1024];
                _stream.read(&mut rx_bytes).expect("TODO: panic message");
                let string = std::str::from_utf8(&rx_bytes).expect("valid utf8");


                let request = HttpRequest::parse_request(string);
                if let Ok(re) = request {
                    let res = router.match_request(re, &DIRECTORY);
                    _stream.write(res.to_string().as_bytes()).expect("zadzd");
                }
                 else {
                    println!("Parse Error");
                }
                    _stream.shutdown(Both).unwrap();}
                );
            }
            Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
