use std::str::FromStr;

// can give a number to each variant
// if POST = 5, then PUT = 6, etc.
// enum is a type that can be one of a few different variants, like a union in C
#[derive(Debug)]
pub enum Method {
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

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "DELETE" => Ok(Self::DELETE),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      "PATCH" => Ok(Self::PATCH),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError;
