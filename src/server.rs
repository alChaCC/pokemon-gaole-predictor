// every file in the src directory is a module
// the server module is a child of the crate root module

// Path: src/server.rs

// by default every thing in Rust is private
// pub keyword makes the struct public

use std::net::TcpListener;
use std::io::Read;
// use crate is similar to use super to access root module
use crate::http::Request;
pub struct Server {
    addr: String,
}

impl Server {
  // associated function(static method)
  // Self is an alias for the type that we're implementing the methods on (Server in this case).
  pub fn new(addr: String) -> Self {
      // because we have the same name for the struct field and the function parameter, we can use
      // the field init shorthand syntax
      Self { addr }
  }

  // method (instance method)
  // self is a reference to the instance of the struct
  // if we want to mutate the instance, we can use &mut self
  // if we want to take ownership of self, we can use self
  // if we want to allow the method to be called on an immutable or mutable instance, we can use &self or &mut self
  pub fn run(self) {
      println!("Listening on {}", self.addr);

      // this is recoverable error, but we don't want to recover from it so we use unwrap.
      // so if the result is Err, the program will terminate.
      // if we want to handle the error, we can use match
      let listener = TcpListener::bind(&self.addr).unwrap();

      // infinite loop
      // no need to use while true
      // loop can be labeled !!!
      // syntax: 'label: loop { ... }
      // ex:
      // 'outer: loop {
      //    loop {
      //      break 'outer;
      //    }
      // }
      loop {
          // match is an expression that returns a value
          // the code will not compile if we don't handle ALL the possible cases
          // match can also match against ranges of values
          // ex:
          // match number {
          //   1 => println!("one"),
          //   2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
          //   13..=19 => println!("a teen"),
          //   _ => println!("not interesting"),
          // }
          match listener.accept() {
              Ok((mut stream, _)) => {
                println!("Accepted a connection");
                // read the request
                // we can use a buffer to store the data
                // we need to specify the type of the buffer(array)
                // here we speicify buf is an array of 1024 bytes with default value 0
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        // convert the buffer to a string
                        // String::from_utf8_lossy will replace any invalid UTF-8 sequences with �
                        // if we want to handle invalid UTF-8 sequences, we can use String::from_utf8, it will terminate the program if it encounters invalid UTF-8 sequences
                        println!("Request: {}", String::from_utf8_lossy(&buf));
                        match Request::try_from(&buf[..]) {
                            Ok(request) => {dbg!(request);}
                            Err(e) => println!("Failed to parse a request: {}", e),
                        }
                    }
                    Err(e) => println!("Failed to read from connection: {}", e),
                }
              }
              Err(e) => println!("Failed to establish a connection: {}", e),
          }
      }
  }
}
