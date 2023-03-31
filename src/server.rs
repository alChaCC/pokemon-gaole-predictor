// every file in the src directory is a module
// the server module is a child of the crate root module

// Path: src/server.rs

// by default every thing in Rust is private
// pub keyword makes the struct public

use std::net::TcpListener;

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
              Ok((stream, _)) => {
                println!("Accepted a connection");
              }
              Err(e) => println!("Failed to establish a connection: {}", e),
          }
      }
  }
}
