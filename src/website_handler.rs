use super::server::Handler;
use std::fs;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // ok() is a handy method that converts a Result to an Option
        // if the Result is Ok, it will return Some
        // if the Result is Err, it will return None
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack detected: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        // match request.method() {
        //     Some(&crate::http::Method::GET) => {
        //         match request.path() {
        //             "/" => crate::http::Response::new(
        //                 crate::http::StatusCode::Ok200,
        //                 Some("<h1>Welcome</h1>".to_string()),
        //             ),
        //             "/hello" => crate::http::Response::new(
        //                 crate::http::StatusCode::Ok200,
        //                 Some("<h1>Hello</h1>".to_string()),
        //             ),
        //             _ => crate::http::Response::new(
        //                 crate::http::StatusCode::NotFound404,
        //                 Some("<h1>Not Found</h1>".to_string()),
        //             ),
        //         }
        //     }
        //     _ => crate::http::Response::new(
        //         crate::http::StatusCode::NotFound404,
        //         Some("<h1>Not Found</h1>".to_string()),
        //     ),
        // }
        let response = match request.method() {
            crate::http::Method::GET => match request.path() {
                "/" => crate::http::Response::new(
                    crate::http::StatusCode::Ok200,
                    self.read_file("index.html"),
                ),
                "/hello" => crate::http::Response::new(
                    crate::http::StatusCode::Ok200,
                    Some("<h1>Hello</h1>".to_string()),
                ),
                // we can match path and pass the path to read_file
                // but here has security issue
                // if we pass ../../etc/passwd, it will read the file
                // so we need to check if the path is valid
                path => match self.read_file(path) {
                    Some(content) => crate::http::Response::new(
                        crate::http::StatusCode::Ok200,
                        Some(content),
                    ),
                    None => crate::http::Response::new(
                        crate::http::StatusCode::NotFound404,
                        Some("<h1>Not Found</h1>".to_string()),
                    ),
                },
            },
            _ => crate::http::Response::new(
                crate::http::StatusCode::NotFound404,
                Some("<h1>Not Found</h1>".to_string()),
            ),
        };
        response
    }
}
