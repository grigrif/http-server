use std::collections::HashMap;

pub struct Response {
    response_code: usize,
    headers: HashMap<String, String>,
    body: Option<String>
}



impl Response {
    pub fn to_string(self) -> String {
        let mut lines = Vec::new();
        let first_line = match self.response_code {
            200 => {
                "HTTP/1.1 200 OK"
            },
            201 => {
                "HTTP/1.1 200 OK"
            },
            404 => {
                "HTTP/1.1 404 Not Found"
            }

            _ => {
                "HTTP/1.1 500 Not Found"
            }
        }.to_string();

        lines.push(first_line);

        for (k, v) in self.headers {
            lines.push(format!("{}: {}", &k, &v));
        }
        lines.push("".to_string());
        if let Some(body) = self.body {
            lines.push(body);
            return lines.join("\r\n");

        }

        lines.join("\r\n")

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
        m.body = Some(body.to_string());
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
        m.body = Some(body);
        return m;
    }

}
