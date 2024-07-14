use std::collections::HashMap;
use flate2::write::GzEncoder;
use nom::AsBytes;

#[derive(Clone)]
pub struct Response {
    response_code: usize,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}



impl Response {
    pub fn to_string(self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let first_line = match self.response_code {
            200 => {
                "HTTP/1.1 200 OK\r\n"
            },
            201 => {
                "HTTP/1.1 201 Created\r\n"
            },
            404 => {
                "HTTP/1.1 404 Not Found\r\n"
            }

            _ => {
                "HTTP/1.1 500 Not Found\r\n"
            }
        }.to_string();

        buffer.extend(first_line.as_bytes());

        for (k, v) in &(self.headers) {
            buffer.extend(format!("{}: {}", &k, &v).as_bytes());
            buffer.extend("\r\n".to_string().as_bytes());
        }
        buffer.extend("\r\n".to_string().as_bytes());


        if let Some(body) = self.body {
            println!("body");
            buffer.extend(body);

        }

        buffer

    }

    fn new() -> Response {
        Response {
            response_code: 0,
            headers: HashMap::new(),
            body: None
        }}


    pub fn not_found() -> Response {
        let mut m = Response::new();
        m.response_code = 404;
        return m;
    }
    pub fn ok() -> Response {
        let mut m = Response::new();
        m.response_code = 200;
        return m;
    }
    pub fn ok_with_body(body: &str) -> Response {
        let mut m = Response::new();
        m.response_code = 200;
        m.body = Some(Vec::from(body.as_bytes()));
        m.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        println!("{body}");
        m.headers.insert("Content-Length".to_string(), body.len().to_string());
        return m;
    }
    pub fn ok_201() -> Response {
        let mut m = Response::new();
        m.response_code = 201;
        return m;
    }

    pub fn octet_stream(body: String) -> Response {
        let mut m = Response::new();
        m.response_code = 200;
        m.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
        m.headers.insert("Content-Length".to_string(), body.len().to_string());
        m.body = Some(body.into_bytes());
        return m;
    }

}
