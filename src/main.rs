// we tell compiler to ignore dead code warnings
#![allow(dead_code)]

// include the server module
mod server;
use server::Server; // once we include the server module, we can use the Server struct

mod http; // although we don't have http.rs, we can create mod.rs in http directory
use http::Request;
use http::Method;

mod website_handler;
use website_handler::WebsiteHandler;

fn main() {
    // CARGO_MANIFEST_DIR is a special environment variable that is set by Cargo
    // it is the path to the directory that contains the Cargo.toml file
    // it will be different for each project
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Serving path: {}", public_path);

    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string.to_string());
    server.run(WebsiteHandler::new(public_path));
}
