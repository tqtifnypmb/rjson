use std::fmt::{self, Debug, Formatter};

pub struct Number {
    inner: Inner
}

enum Inner {
    Postive(u64),
    Negative(i64),
    Float(f64)
}

impl Number {

    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            Some(Number { inner: Inner::Float(f) })
        } else {
            None
        }
    }

    pub fn from_i64(f: i64) -> Option<Number> {
        if f < 0 {
            Some(Number{ inner: Inner::Negative(f) })
        } else {
            Some(Number{ inner: Inner::Postive(f as u64) })
        }
    }

    pub fn is_u64(&self) -> bool {
        match self.inner {
            Inner::Postive(_) => true,
            _ => false
        }
    }

    pub fn is_i64(&self) -> bool {
        match self.inner {
            Inner::Negative(_) => true,
            _ => false
        }
    }

    pub fn is_f64(&self) -> bool {
        match self.inner {
            Inner::Float(_) => true,
            _ => false
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self.inner {
            Inner::Postive(v) => Some(v),
            _ => None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self.inner {
            Inner::Negative(v) => Some(v),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self.inner {
            Inner::Float(v) => Some(v),
            _ => None
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.inner {
            Inner::Postive(v) => f.debug_tuple("Positive").field(&v).finish(),
            Inner::Negative(v) => f.debug_tuple("Negative").field(&v).finish(),
            Inner::Float(v) => f.debug_tuple("Float").field(&v).finish()
        }
    }
}

impl Default for Number {
    
    fn default() -> Self {
        Number { inner: Inner::Postive(0) }
    }
}