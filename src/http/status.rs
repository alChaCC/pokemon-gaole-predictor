use std::fmt::{Display, Formatter, Result as FmtResult};

// if a type impl Copy, then it will need to impl Clone too
// once add Clone we can use *self as u16
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok200 = 200,
  NotFound404 = 404,
  BadRequest400 = 400,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok200 => "OK",
      Self::NotFound404 => "Not Found",
      Self::BadRequest400 => "Bad Request",
    }
  }
}

impl Display for StatusCode{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    // use as for type casting
    write!(f, "{} {}", *self as u16, self.reason_phrase())
  }
}

// Clone is a trait that allows us to make a copy of the value
// Copy is a trait that is a subset of Clone
