use std::collections::{BTreeMap, HashMap};
use std::iter::Map;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TransportValue {
    pub name: String,
    pub values: HashMap<String, Value>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    String(String),
    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}

impl Default for TransportValue {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            values: HashMap::from([("id".to_string(), Value::String(Uuid::new_v4().to_string()))])
        }
    }
}

impl TransportValue {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            values: HashMap::from([("id".to_string(), Value::String(Uuid::new_v4().to_string()))])
        }
    }
    pub fn set_value(&mut self, key: &str, value: Value) {
        self.values.insert(String::from(key), value);
    }
}


impl From<u8> for Value {
    fn from(value: u8) -> Value {
        Value::U8(value)
    }
}
impl From<i8> for Value {
    fn from(value: i8) -> Value {
        Value::I8(value)
    }
}
impl From<u16> for Value {
    fn from(value: u16) -> Value {
        Value::U16(value)
    }
}
impl From<i16> for Value {
    fn from(value: i16) -> Value {
        Value::I16(value)
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Value {
        Value::U32(value)
    }
}
impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::I32(value)
    }
}
impl From<u64> for Value {
    fn from(value: u64) -> Value {
        Value::U64(value)
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::I64(value)
    }
}
// impl From<u128> for Value {
//     fn from(value: u128) -> Value {
//         Value::U128(value)
//     }
// }
// impl From<i128> for Value {
//     fn from(value: i128) -> Value {
//         Value::I128(value)
//     }
// }

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value::String(value.to_string())
    }
}

impl<T> From<Vec<T>> for Value
    where
        Value: From<T>,
{
    fn from(x: Vec<T>) -> Value {
        x.into_iter().map(|item| item.into()).collect::<Vec<Value>>().into()
    }
}

impl<T> From<Option<T>> for Value
    where
        Value: From<T>,
{
    fn from(x: Option<T>) -> Value {
        match x {
            Some(x) => x.into(),
            None => Value::Null,
        }
    }
}