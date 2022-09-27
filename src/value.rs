use serde::{Deserialize, Serialize};
use serde_json::Value as SerdeValue;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(u64),
    String(String),
    Array(Vec<Value>),
    Object(
        // HashMap<String, Value>
        String,
    ),
}

impl From<SerdeValue> for Value {
    fn from(value: SerdeValue) -> Self {
        match value {
            SerdeValue::Null => Value::Null,
            SerdeValue::Bool(bool) => Value::Bool(bool),
            SerdeValue::Number(number) => 
                match number.as_u64() {
                    Some(number) => Value::Number(number),
                    None => Value::String(number.to_string()),
                },
            SerdeValue::String(string) => Value::String(string),
            SerdeValue::Array(vec) => {
                Value::Array(vec.into_iter().map(|x| Self::from(x)).collect())
            }
            SerdeValue::Object(map) => {
                // Value::Object(map.into_iter().map(|x| (x.0, Self::from(x.1))).collect())
                Value::Object("test".to_string())
            }
        }
    }
}

impl Into<SerdeValue> for Value {
    fn into(self) -> SerdeValue {
        match self {
            Value::Null => SerdeValue::Null,
            Value::Bool(bool) => SerdeValue::Bool(bool),
            Value::Number(number) => SerdeValue::Number(serde_json::Number::from(number)),
            Value::String(string) => SerdeValue::String(string),
            Value::Array(vec) => {
                SerdeValue::Array(vec.into_iter().map(|x| Self::into(x)).collect())
            },
            Value::Object(map) => {
                // SerdeValue::Object(map.into_iter().map(|x| (x.0, Self::into(x.1))).collect())
                SerdeValue::String(map)
            }
        }
    }
}
