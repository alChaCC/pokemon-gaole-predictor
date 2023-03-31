fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string.to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // associated function(static method)
    // Self is an alias for the type that we're implementing the methods on (Server in this case).
    fn new(addr: String) -> Self {
        // because we have the same name for the struct field and the function parameter, we can use
        // the field init shorthand syntax
        Self { addr }
    }

    // method (instance method)
    // self is a reference to the instance of the struct
    // if we want to mutate the instance, we can use &mut self
    // if we want to take ownership of self, we can use self
    // if we want to allow the method to be called on an immutable or mutable instance, we can use &self or &mut self
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

/*
GET /user?id=1 HTTP/1.1\r\n
HEADERS\r\n
BODY
 */
// Rust not allow null values, but it does have an enum that can encode the concept of a value being present or absent
// Option<String> is an enum used to represent the possibility of a String value or the absence of a value
// It has two variants: Some and None
// Some is a tuple struct that wraps a String
// None is a unit struct that doesn't hold any value
// Option<T> will automatically be imported into scope when we use it
struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// can give a number to each variant
// if POST = 5, then PUT = 6, etc.
// enum is a type that can be one of a few different variants, like a union in C
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
