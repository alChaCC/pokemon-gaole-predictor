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
use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
      // unimplemented!() is a macro that will panic when called
      unimplemented!()
    }
}

// trait is a way to define shared behavior in an abstract way, similar to interface in other languages like Java, Go, etc.
// we can use trait bounds to specify that a generic type can be any type that implements a trait
// we can implement TryFrom "trait" for Request
// TryFrom is a trait in the standard library that allows us to define a conversion between types
// TryFrom is the opposite of From
// TryFrom is used for fallible conversions, and From is used for infallible conversions
impl TryFrom<&[u8]> for Request {
  // must give the type of the Error
  type Error = ParseError;


  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    // way 1
    // match str::from_utf8(buf) {
    //   Ok(request) => {},
    //   Err(_) => return Err(ParseError::InvalidEncoding),
    // }

    // way 2
    // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
    //   Ok(request) => {},
    //   Err(e) => return Err(e),
    // }

    // way 3 (common pattern in Rust)
    // Rust uses the ? operator to return early from a function if an Err variant is found
    // ? can only be used in functions that return Result or Option
    //  let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

    // way 4
    // if remove or(Err(ParseError::InvalidEncoding)), then it will return Result<&str, Utf8Error>
    // so buf need to implement From<Utf8Error> for ParseError
    // please check line: 76
    let request = str::from_utf8(buf)?;
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

// impl Error must be implemented Display and Debug
impl Error for ParseError {}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}
