// can give a number to each variant
// if POST = 5, then PUT = 6, etc.
// enum is a type that can be one of a few different variants, like a union in C
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
