use ethnum::{I256, U256};
use serde::{Deserialize, Serialize};
use serde_json::Value as SerdeValue;

use crate::errors::RetrieveValueError;

#[derive(Clone, Serialize, Deserialize, Debug, Eq)]
pub enum Value {
    Int8(I256),
    Int16(I256),
    Int32(I256),
    Int64(I256),
    Int128(I256),
    Int256(I256),
    UInt8(U256),
    UInt16(U256),
    UInt32(U256),
    UInt64(U256),
    UInt128(U256),
    UInt256(U256),
    Binary(Vec<u8>),
    Array(Vec<Value>),
}

impl TryFrom<SerdeValue> for Value {
    type Error = RetrieveValueError;
    fn try_from(value: SerdeValue) -> Result<Self, Self::Error> {
        let serialized_value = serde_json::to_string(&value)?;
        let deserialized_value: Value = serde_json::from_str(&serialized_value)?;
        Ok(deserialized_value)
    }
}

impl TryFrom<Value> for SerdeValue {
    type Error = RetrieveValueError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let serialized_value = serde_json::to_string(&value)?;
        let deserialized_value: SerdeValue = serde_json::from_str(&serialized_value)?;
        Ok(deserialized_value)
    }
}

//todo: change to error when different types
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (
                Int8(a) | Int16(a) | Int32(a) | Int64(a) | Int128(a) | Int256(a),
                Int8(b) | Int16(b) | Int32(b) | Int64(b) | Int128(b) | Int256(b),
            ) => a == b,
            (
                UInt8(a) | UInt16(a) | UInt32(a) | UInt64(a) | UInt128(a) | UInt256(a),
                UInt8(b) | UInt16(b) | UInt32(b) | UInt64(b) | UInt128(b) | UInt256(b),
            ) => a == b,
            (Binary(a), Binary(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            _ => false,
        }
    }
}

// todo: change to error when different types
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (
                Int8(a) | Int16(a) | Int32(a) | Int64(a) | Int128(a) | Int256(a),
                Int8(b) | Int16(b) | Int32(b) | Int64(b) | Int128(b) | Int256(b),
            ) => Some(a.cmp(b)),
            (
                UInt8(a) | UInt16(a) | UInt32(a) | UInt64(a) | UInt128(a) | UInt256(a),
                UInt8(b) | UInt16(b) | UInt32(b) | UInt64(b) | UInt128(b) | UInt256(b),
            ) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

impl Value {
    //todo: change to error when different types
    pub fn is_in(&self, value: &Value) -> bool {
        use Value::*;
        match (self, value) {
            (Array(_), Array(_)) => false,
            (other, Array(vector)) => vector.iter().any(|x| x.eq(other)),
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
    fn value_convertion_happy_flow() {
        let value = Value::Array(vec![
            Value::Int8(I256::from(-123i8)),
            Value::Int16(I256::from(-123i16)),
            Value::Int32(I256::from(-123i32)),
            Value::Int64(I256::from(-123i64)),
            Value::Int128(I256::from(-123i128)),
            Value::Int256(I256::from(-123i32)),
            Value::UInt8(U256::from(255u8)),
            Value::UInt16(U256::from(255u16)),
            Value::UInt32(U256::from(255u32)),
            Value::UInt64(U256::from(255u64)),
            Value::UInt128(U256::from(255u128)),
            Value::UInt256(U256::from(255u32)),
            Value::Binary(vec![8, 7, 6]),
            Value::Array(vec![
                Value::Binary(vec![1, 2, 3]),
                Value::Binary(vec![4, 5, 6]),
                Value::Binary(vec![7, 8, 9]),
            ]),
        ]);
        let serde_value: SerdeValue = value.clone().try_into().unwrap();
        let converted_value: Value = serde_value.try_into().unwrap();
        assert_eq!(value, converted_value);
    }

    #[test]
    #[should_panic(expected = "SerializationError(Error(\"expected value\", line: 1, column: 1))")]
    fn value_convertion_random_serdevalue_to_value() {
        let serialized_value = serde_json::to_string(&[5; 5]).unwrap();
        let serde_value: SerdeValue = serde_json::from_str(&serialized_value).unwrap();

        let _converted_value: Value = serde_value.try_into().unwrap();
    }

    #[test]
    fn is_equal_or_not() {
        let int64 = Value::Int64(I256::from(-123i64));
        let ne_int64 = Value::Int64(I256::from(123i64));
        let eq_int64 = int64.clone();
        let eq_int128 = Value::Int128(I256::from(-123i128));

        assert!(int64.ne(&ne_int64));
        assert!(int64.eq(&eq_int64));
        assert!(int64.eq(&eq_int128));

        let uint64 = Value::UInt64(U256::from(123u64));
        let ne_uint64 = Value::UInt64(U256::from(456u64));
        let eq_uint64 = uint64.clone();
        let eq_uint128 = Value::UInt128(U256::from(123u128));

        assert!(uint64.ne(&ne_uint64));
        assert!(uint64.eq(&eq_uint64));
        assert!(uint64.eq(&eq_uint128));

        let binary = Value::Binary(vec![8, 7, 6]);
        let ne_binary = Value::Binary(vec![1, 2, 3]);
        let eq_binary = binary.clone();

        assert!(binary.ne(&ne_binary));
        assert!(binary.eq(&eq_binary));

        let array = Value::Array(vec![
            int64.clone(),
            uint64.clone(),
            binary.clone(),
            Value::Array(vec![binary.clone(), ne_binary.clone()]),
        ]);
        let ne_array = Value::Array(vec![int64.clone(), uint64.clone()]);
        let eq_array = array.clone();

        assert!(array.ne(&ne_array));
        assert!(array.eq(&eq_array));

        assert!(array.ne(&binary));
        assert!(ne_int64.ne(&ne_uint64));
    }

    #[test]
    fn is_in_or_not() {
        let int = Value::Int128(I256::from(-123i128));

        let uint = Value::UInt128(U256::from(123u128));

        let binary = Value::Binary(vec![8, 7, 6]);

        let array = Value::Array(vec![int.clone(), uint.clone(), binary.clone()]);

        assert!(int.is_in(&array));
        assert!(int.not_in(&int));
        assert!(int.not_in(&uint));
        assert!(int.not_in(&binary));

        assert!(uint.is_in(&array));
        assert!(uint.not_in(&int));
        assert!(uint.not_in(&uint));
        assert!(uint.not_in(&binary));

        assert!(binary.is_in(&array));
        assert!(binary.not_in(&int));
        assert!(binary.not_in(&uint));
        assert!(binary.not_in(&binary));

        assert!(array.not_in(&array));
        assert!(array.not_in(&int));
        assert!(array.not_in(&uint));
        assert!(array.not_in(&binary));
    }

    #[test]
    fn comparison() {
        let int32 = Value::Int32(I256::from(-123i32));
        let gt_int64 = Value::Int64(I256::from(-120i64));
        let lt_int128 = Value::Int128(I256::from(-125i128));

        assert!(gt_int64.gt(&int32));
        assert!(lt_int128.lt(&int32));

        let uint32 = Value::UInt32(U256::from(123u32));
        let gt_uint64 = Value::UInt64(U256::from(125u64));
        let lt_uint128 = Value::UInt128(U256::from(120u128));

        assert!(gt_uint64.ge(&uint32));
        assert!(lt_uint128.le(&uint32));

        let binary = Value::Binary(vec![8, 7, 6]);
        let le_binary = Value::Binary(vec![1, 2, 3]);

        assert!(!binary.le(&le_binary));
        assert!(!binary.gt(&le_binary));

        let array = Value::Array(vec![
            int32.clone(),
            uint32.clone(),
            binary.clone(),
            Value::Array(vec![binary.clone(), le_binary.clone()]),
        ]);
        let le_array = Value::Array(vec![int32.clone(), uint32.clone()]);

        assert!(!array.le(&le_array));
        assert!(!array.gt(&le_array));

        assert!(!int32.le(&uint32));
        assert!(!int32.lt(&binary));
        assert!(!int32.ge(&array));

        assert!(!uint32.lt(&binary));
        assert!(!uint32.ge(&array));

        assert!(!binary.lt(&array));
    }
}
