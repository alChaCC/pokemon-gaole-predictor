use super::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Response{
  status_code: StatusCode,
  body: Option<String>
}

impl Response {
  pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
    Self {
      status_code,
      body,
    }
  }

  // write directly to the stream
  // this is the way we want to do
  // here use &mut impl std::io::Write to make it more generic
  // so we can pass in TcpStream or any other type that impl std::io::Write
  // it called static dispatch because the compiler knows the type of stream at compile time
  // in compile time it will know the type of stream and will generate the code for that type
  // so it's faster
  // example: if we pass in TcpStream, it will generate the code for TcpStream
  // ex: pub fn sendTcpStream(&self, stream: &mut TcpStream) -> IoResult<()>
  // if we pass in File, it will generate the code for File
  // ex: pub fn sendFile(&self, stream: &mut File) -> IoResult<()>
  // if we pass in Vec<u8>, it will generate the code for Vec<u8>
  pub fn send(&self, stream: &mut impl std::io::Write) -> IoResult<()> {
    let body = match &self.body {
      Some(b) => b,
      None => "",
    };
    write!(
      stream,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      body
    )
  }
}

// way 1: will allocate memory for the string
// impl Display for Response {
//   fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//     let body = match &self.body {
//       Some(b) => b,
//       None => "",
//     };
//     write!(
//       f,
//       "HTTP/1.1 {} {}\r\n\r\n{}",
//       self.status_code,
//       self.status_code.reason_phrase(),
//       body
//     )
//   }
// }
