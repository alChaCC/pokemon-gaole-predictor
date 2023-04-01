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
