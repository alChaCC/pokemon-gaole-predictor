// every file in the src directory is a module
// the server module is a child of the crate root module

// Path: src/server.rs

// by default every thing in Rust is private
// pub keyword makes the struct public
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
  }
}
