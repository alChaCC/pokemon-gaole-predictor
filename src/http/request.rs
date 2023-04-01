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
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString};

// use &str because the request content is immutable
// we can use String, but &str is more efficient because it doesn't require memory allocation
// becareful with lifetimes!!!
// if Request live longer than the buffer(&str), we will have a dangling pointer
// so we need to specify the lifetime of the buffer(&str)

// *******lifetime is super unique in Rust********
// need to think about the lifetime of the reference
// and specify the lifetime of the reference
// so compiler can know how long the reference will live when doing static checking
// we can use 'a to specify the lifetime of the reference
// it's like a meta variable for compiler to check the lifetime of the reference

// lifetime can help us avoid dangling pointer in Rust
// we can use lifetime to specify the lifetime of the reference
// It allows us to communicate to the compiler that some references are "related" and are expected to share the same lifetime.
pub struct Request<'buf> {
  path: &'buf str,
  query_string: Option<QueryString<'buf>>,
  method: Method,
}

impl Request<'_> {
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
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  // must give the type of the Error
  type Error = ParseError;

  fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
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

    // way 1
    // match get_next_word(request) {
    //     Some((method, request)) => match method {
    //         "GET" => {},
    //         _ => return Err(ParseError::InvalidMethod),
    //     },
    //     None => return Err(ParseError::InvalidRequest),
    // }

    // way 2
    let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocal, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

    if protocal != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    // because we implement FromStr for Method, so we can use parse() to convert a string
    // and because we implement From<Method> for ParseError, so we can use parse()? to convert a string
    let method: Method = method.parse()?;
    let mut query_string = None;
    // way 1
    // match path.find('?') {
    //   Some(i) => {
    //     query_string = Some(&path[i + 1..]);
    //     path = &path[..i];
    //   },
    //   None => {},
    // }

    // way 2
    // let q = path.find('?');
    // if q.is_some() {
    //   let i = q.unwrap();
    //   query_string = Some(&path[i + 1..]);
    //   path = &path[..i];
    // }

    // way 3 (common pattern in Rust), better solution
    if let Some(i) = path.find('?') {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    Ok(Self {
      path: path,
      query_string: query_string,
      method,
    })
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

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  // way 1
  // let mut iter = request.chars();
  // loop {
  //   let item = iter.next();
  //   match item {
  //     Some(' ') => continue,
  //     Some(_) => break,
  //     None => return None,
  //   }
  // }

  // way 2
  // use enumerate to get the index of the character
  for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      // it's dangerous to use &request[i+1..] because string could be emoji
      // but here we know we're skip a space which is for sure 1 byte so it's safe to to +1
      return Some((&request[..i], &request[i + 1..]));
    }
  }
  None
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
