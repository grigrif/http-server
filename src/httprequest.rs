use std::collections::HashMap;
use std::fmt::Error;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    http_version: String,
    pub headers: HashMap<String, String>,
    pub(crate) body: String
}

impl HttpRequest {
    pub fn parse_request(string: &str) -> Result<HttpRequest, Error> {
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
            let (a, b) = l.split_once(": ").ok_or(Error)?;
            map.insert(a.to_string(), b.to_string());
        }
        let body: String = lines.collect();
        let (body, _) = body.split_once("\0").ok_or(Error)?;
        Ok (HttpRequest {
            method: method.to_string(), path: path.to_string(), http_version: http_version.to_string(), headers: map, body: body.to_string()
        })
    }

    pub fn headers(self) -> HashMap<String, String>  {self.headers}
}
