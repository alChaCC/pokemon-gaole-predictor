use std::collections::HashMap;
#[derive(Debug)]
pub struct QueryString<'buf>  {
  data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
// can not implement TryFrom<&'buf str> for QueryString<'buf> because TryFrom is not support lifetime
impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut val = "";

      if let Some(i) = sub_str.find('=') {
        key = &sub_str[..i];
        val = &sub_str[i + 1..];
      }

      // entry can
      // closure will be called if the key is present
      data.entry(key)
        .and_modify(|existing: &mut Value| match existing {
          Value::Single(prev_val) => {
            // vec![] is a macro, not a function and it can create a vector
            // exsiting is just a reference, so we need to dereference it(*...) to get the value
            *existing = Value::Multiple(vec![prev_val, val]);
          }
          Value::Multiple(vec) => vec.push(val),
        })
        .or_insert(Value::Single(val));
    }

    QueryString { data }
  }
}
