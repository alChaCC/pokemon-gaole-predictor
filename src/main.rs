// include the server module
mod server;
use server::Server; // once we include the server module, we can use the Server struct

mod http; // although we don't have http.rs, we can create mod.rs in http directory
use http::Request;
use http::Method;

fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string.to_string());
    server.run();
}
