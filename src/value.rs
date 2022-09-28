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
    Object(HashMap<String, Value>),
}

impl From<SerdeValue> for Value {
    fn from(value: SerdeValue) -> Self {
        match value {
            SerdeValue::Null => Value::Null,
            SerdeValue::Bool(bool) => Value::Bool(bool),
            SerdeValue::Number(number) => match number.as_u64() {
                Some(number) => Value::Number(number),
                None => Value::String(number.to_string()),
            },
            SerdeValue::String(string) => Value::String(string),
            SerdeValue::Array(vec) => {
                Value::Array(vec.into_iter().map(|x| Self::from(x)).collect())
            }
            SerdeValue::Object(map) => {
                Value::Object(map.into_iter().map(|x| (x.0, Self::from(x.1))).collect())
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
            }
            Value::Object(map) => {
                SerdeValue::Object(map.into_iter().map(|x| (x.0, Self::into(x.1))).collect())
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Null, Null) => true,
            (Bool(a), Bool(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (Number(a), Number(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

impl Value {
    pub fn is_in(&self, value: &Value) -> bool {
        use Value::*;
        match (self, value) {
            (Array(_vector_1), Array(_vector_2)) => false,
            (Object(_object), Array(_vector)) => false,
            (other, Array(vector)) => match vector.iter().find(|x| x.eq(&other)) {
                Some(_) => true,
                None => false,
            },
            (_, _) => false,
        }
    }

    pub fn not_in(&self, value: &Value) -> bool {
        !self.is_in(value)
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn test_is_in() {
        let vector = Value::Array(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
            Value::Number(4),
        ]);

        let number = Value::Number(3);
        assert!(number.is_in(&vector));

        let boolean = Value::Bool(true);
        assert!(!boolean.is_in(&vector));

        let vector_2 = vector.clone();
        assert!(!vector_2.is_in(&vector));

        let object = Value::Object(HashMap::from([(String::from("test"), Value::Number(1))]));
        assert!(!object.is_in(&vector));

        assert!(!boolean.is_in(&number));
    }

    #[test]
    fn test_not_in() {
        let vector = Value::Array(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
            Value::Number(4),
        ]);

        let number = Value::Number(5);
        assert!(number.not_in(&vector));

        let boolean = Value::Bool(true);
        assert!(boolean.not_in(&vector));

        let vector_2 = vector.clone();
        assert!(vector_2.not_in(&vector));

        let object = Value::Object(HashMap::from([(String::from("test"), Value::Number(1))]));
        assert!(object.not_in(&vector));

        assert!(boolean.not_in(&number));
    }
}
