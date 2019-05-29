use super::Value;
use std::ops;

pub trait Index {

    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value>;

    fn index_mut_into<'a>(&self, v: &'a mut Value) -> &'a mut Value;
}

impl Index for usize {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        match *v {
            Value::Array(ref arr) => arr.get(*self),
            _ => None
        }
    }

    fn index_mut_into<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        match *v {
            Value::Array(ref mut arr) => arr.get_mut(*self).unwrap_or_else(|| {
                panic!();
            }),
            _ => panic!()
        }
    }
}

impl Index for str {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        match *v {
            Value::Object(ref map) => map.get(self),
            _ => None
        }
    }

    fn index_mut_into<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        match *v {
            Value::Object(ref mut map) => map.get_mut(self).unwrap_or_else(|| {
                panic!()
            }),
            _ => panic!()
        }
    }
}

impl Index for String {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        self[..].index_into(v)
    }

    fn index_mut_into<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        self[..].index_mut_into(v)
    }
}

impl<I> ops::Index<I> for Value
where I: Index 
{
    type Output = Value;

    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value
where I: Index 
{
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_mut_into(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn int_index() {
        let p = vec!["abc", "def"];
        let value: Value = p.into();

        match value[0] {
            Value::String(ref s) => assert_eq!(s, "abc"),
            _ => assert!(false)
        }
    }

    #[test]
    fn string_index() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("abc".to_string(), "def".into());

        let value: Value = map.into();
        match value["abc".to_string()] {
            Value::String(ref s) => assert_eq!(s, "def"),
            _ => assert!(false)
        }
    }
}