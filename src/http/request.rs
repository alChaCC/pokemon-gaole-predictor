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
pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}
