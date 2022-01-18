use std::collections::{BTreeMap, HashMap};
use std::iter::Map;
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
    U128(u128),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}

impl Default for TransportValue {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            values: HashMap::default()
        }
    }
}