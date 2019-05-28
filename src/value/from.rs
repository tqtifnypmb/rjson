use super::Value;
use std::collections::HashMap;
use super::Number;

impl From<bool> for Value {

    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<'a> From<&'a str> for Value {

    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<HashMap<String, Value>> for Value {

    fn from(obj: HashMap<String, Value>) -> Self {
        Value::Object(obj)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {

    fn from(v: Vec<T>) -> Self {
        Value::Array(v.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {

    fn from(v: &'a [T]) -> Self {
        Value::Array(v.into_iter().cloned().map(Into::into).collect())
    }
}

impl From<f32> for Value {

    fn from(v: f32) -> Self {
        From::from(v as f64)
    }
}

impl From<f64> for Value {

    fn from(v: f64) -> Self {
        if let Some(number) = Number::from_f64(v) {
            Value::Number(number)
        } else {
            Value::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bool() {
        let value: Value = true.into();
        if let Value::Bool(v) = value {
            assert_eq!(v, true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn from_f64() {
        let value: Value = (3.14 as f64).into();
        if let Value::Number(n) = value {
            assert!(n.is_f64());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn from_array() {
        let p = vec!["abc", "def"];
        let value: Value = p.into();
        if let Value::Array(v) = value {
            v.into_iter().for_each(|x|{
                println!("{}", x);
            });
        } else {
            assert!(false);
        }
    }

    #[test]
    fn from_slice() {
        let p = vec!["true", "false", "true", "false"];
        let value: Value = p[0 .. 1].into();
        if let Value::Array(v) = value {
            v.into_iter().for_each(|x|{
                println!("{}", x);
            });
        } else {
            assert!(false);
        }
    }
}