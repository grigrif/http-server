use std::fs::File;
use std::io::{Read, Write};
use crate::httprequest::HttpRequest;
use crate::response::Response;
#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>
}
#[derive(Clone)]
pub struct Route {
    path: String,
    method: String,
    func: fn(&HttpRequest) -> Response
}

pub fn build_route() -> Router {
    let mut router = Router::new();

    router.add_route(
        Route::new("GET", "/user-agent", |req| {
           let binding = req.clone().headers();

            let user_agent = binding.get("User-Agent");
            if let Some(user_agent) = user_agent {
                return Response::ok_with_body(user_agent)
            }
            Response::ok()
        })
    );
    router.add_route(
        Route::new("GET", "/echo/*", |req| {
            Response::ok_with_body(&req.path[6..])
        })
    );
    router.add_route(
        Route::new("GET", "/files/*", |req| {
            let binding = req.clone().headers();
            let str = &req.path[7..];
            let directory = binding.get("directory").unwrap();
            let file = File::open(format!("{}/{}", directory, str));
            if let Ok(mut fe) = file {
                let mut contents = String::new();
                fe.read_to_string(&mut contents).expect("TODO: panic message");
                    Response::ok_with_body(&contents)
                } else {
                    Response::not_found()
                }
        })
    );
    router.add_route(
        Route::new("POST", "/files/*", |req| {
            let str = &req.path[7..];
            let directory = req.headers.get("directory").unwrap();
            let mut file = File::create(format!("{}/{}", directory, str)).unwrap();
            file.write_all(req.body.as_ref()).expect("TODO: panic message");
            Response::ok_201()
        })
    );
    router.add_route(
        Route::new("GET", "/", |_| {
            Response::ok()
        })
    );
    return router
}

fn encoding(http_request: &HttpRequest, response: Response) -> Response {
    if let Some(s) = http_request.headers.get("Accept-Encoding")  {
        if s == "gzip" {
            let mut res = response.clone();
            res.headers.insert("Content-Encoding".parse().unwrap(), "gzip".parse().unwrap());
            return res
        }
    }
    return response;
}
impl Router {
    pub fn match_request(self, http_request: HttpRequest, directory: &String) -> Response {
        let mut h = http_request.clone();
        h.headers.insert("directory".parse().unwrap(), directory.clone());
        for route in self.routes {
            if route.method != http_request.method {
                continue;
            }
            if route.path.ends_with("*") && http_request.path.starts_with(&route.path[..(route.path.len()-1)]){
                return encoding(&http_request, (route.func)(&h));
            }

            if  route.path == http_request.path {
                return encoding(&http_request, (route.func)(&h));
            }
        }
        return Response::not_found();
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }
    pub fn new() -> Router {
        Router {
            routes: Vec::new()
        }
    }
}

impl Route {
    pub fn new(method: &str, path:&str, func: fn(&HttpRequest) -> Response) -> Route {
        Route {
            method: method.to_string(), path: path.to_string(), func
        }
    }
}