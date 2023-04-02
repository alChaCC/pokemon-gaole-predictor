use super::server::Handler;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
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
                    Some("<h1>Welcome</h1>".to_string()),
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
