use super::server::Handler;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let file_path = format!("{}/{}", self.public_path, file_path);
        // ok() is a handy method that converts a Result to an Option
        // if the Result is Ok, it will return Some
        // if the Result is Err, it will return None
        std::fs::read_to_string(file_path).ok()
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
                _ => crate::http::Response::new(
                    crate::http::StatusCode::NotFound404,
                    Some("<h1>Not Found</h1>".to_string()),
                ),
            },
            _ => crate::http::Response::new(
                crate::http::StatusCode::NotFound404,
                Some("<h1>Not Found</h1>".to_string()),
            ),
        };
        response
    }
}
