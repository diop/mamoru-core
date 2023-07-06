use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::blockchain_data::serialize::{deserialize_data, serialize_data};
use crate::ValueError;

/// Holds serialized [`Value`]
#[derive(Clone)]
pub struct ValueData {
    data: Vec<u8>,
}

impl Serialize for ValueData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.data)
    }
}

impl<'de> Deserialize<'de> for ValueData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;

        Ok(Self { data: bytes })
    }
}

impl ValueData {
    pub fn new(value: Value) -> Option<Self> {
        Some(Self {
            data: value.serialize(),
        })
    }
}

impl AsRef<[u8]> for ValueData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

/// Represents variable types passed from blockchains.
/// The end user can work with it via UDFs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    #[serde(rename = "b")]
    Bool(bool),
    #[serde(rename = "u64")]
    U64(u64),
    #[serde(rename = "s")]
    String(String),
    #[serde(rename = "l")]
    List(Vec<Value>),
    #[serde(rename = "st")]
    Struct(StructValue),
}

impl Value {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        serialize_data(self)
    }

    pub(crate) fn from_slice(data: &[u8]) -> Result<Self, ValueError> {
        deserialize_data(data).map_err(|err| ValueError::Deserialize(Box::new(err)))
    }
}

impl Value {
    pub(crate) fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub(crate) fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(value) => Some(*value),
            _ => None,
        }
    }

    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_struct(&self) -> Option<&StructValue> {
        match self {
            Value::Struct(value) => Some(value),
            _ => None,
        }
    }
}

/// Represents Struct types passed from blockchains.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructValue {
    pub(crate) ty: String,
    pub(crate) fields: HashMap<String, Value>,
}

impl StructValue {
    pub fn new(ty: String, fields: HashMap<String, Value>) -> Self {
        Self { ty, fields }
    }

    pub fn add_field(&mut self, key: String, value: Value) -> Option<Value> {
        self.fields.insert(key, value)
    }

    pub fn field_exist(&self, key: &str) -> bool {
        self.fields.get(key).is_some()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore]
    // cargo test -p mamoru-core value::tests::test_value_serialize -- --ignored --nocapture
    fn test_value_serialize() {
        use crate::StructValue;
        use crate::Value;
        use crate::ValueData;
        use std::collections::HashMap;

        let mut fields = HashMap::new();
        fields.insert("b".to_string(), Value::U64(2));
        fields.insert("c".to_string(), Value::String("test".to_string()));
        fields.insert(
            "d".to_string(),
            Value::List(vec![Value::Bool(true), Value::Bool(false)]),
        );

        fields.insert("e".to_string(), Value::String(format!("{:#x}", 110011u128)));

        let struct_value = StructValue::new("test".to_string(), fields);
        let value = Value::Struct(struct_value);
        let value_data = ValueData::new(value).unwrap();
        let bytes = value_data.as_ref();

        dbg!(&bytes);
    }
}
