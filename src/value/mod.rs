use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

pub use super::number::Number;

pub enum Value {
    Null,
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Number(Number)
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Value::Null => f.debug_tuple("Null").finish(),
            Value::Bool(v) => f.debug_tuple("Bool").field(&v).finish(),
            Value::String(ref v) => f.debug_tuple("String").field(v).finish(),
            Value::Array(ref v) => f.debug_tuple("Array").field(v).finish(),
            Value::Object(ref v) => f.debug_tuple("Object").field(v).finish(),
            Value::Number(ref v) => fmt::Debug::fmt(v, f)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Value::Null => write!(f, "Null"),
            Value::Bool(v) => write!(f, "Boolean: {}", v),
            Value::String(ref s) => write!(f, "String: {}", s),
            _ => write!(f, "Not Implemented")
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

mod from;
mod index;