use std::sync::Arc;

use datafusion::arrow::{
    array::{BinaryArray, BooleanArray, Int64Array, StringArray, UInt64Array},
    datatypes::DataType,
};
use ethnum::{i256, u256};

use crate::blockchain_data::evm_value::{int256, uint256};
use crate::blockchain_data::value::Value;

#[macro_export]
macro_rules! udf {
    ($f:ident, ($arr:ident $ret:expr), [$($arg_idx:expr => $arg_names:ident : ($arg_arr_types:ident $arg_types:expr)),*,], $code:expr) => {
        #[allow(unused_parens)]
        pub(crate) fn $f() -> ::datafusion::logical_expr::ScalarUDF {
            use ::datafusion::error::DataFusionError;

            let fun = ::datafusion::physical_plan::functions::make_scalar_function(|args: &[::datafusion::arrow::array::ArrayRef]| {
                $(
                    let $arg_names = ::datafusion::common::downcast_value!(args[$arg_idx], $arg_arr_types);
                )*

                let iter = ::itertools::izip!($($arg_names.iter()),*)
                    .map(|($($arg_names),*)| {
                        match ($($arg_names),*) {
                            ($(Some($arg_names)),*) => $code($($arg_names),*),
                            _ => None,
                        }
                    });

                let array: $arr = iter.collect();

                Ok(Arc::new(array) as ::datafusion::arrow::array::ArrayRef)
            });

            ::datafusion::logical_expr::create_udf(
                stringify!($f),
                vec![$( $arg_types ),*],
                Arc::new($ret),
                ::datafusion::logical_expr::Volatility::Immutable,
                fun,
            )
        }
    };
}

udf!(
    as_uint64,
    (UInt64Array DataType::UInt64),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let value = Value::from_slice(data).ok()?;

        value.as_u64()
    }
);

udf!(
    as_boolean,
    (BooleanArray DataType::Boolean),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let value = Value::from_slice(data).ok()?;

        value.as_bool()
    }
);

udf!(
    as_string,
    (StringArray DataType::Utf8),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let value = Value::from_slice(data).ok()?;

        value.as_str().map(|s| s.to_string())
    }
);

udf!(
    struct_field,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
        1 => field: (StringArray DataType::Utf8),
    ],
    |data, field| {
        let value = Value::from_slice(data).ok()?;
        let field = value.as_struct()?.fields.get(field)?;

        Some(field.serialize())
    }
);

udf!(
    bytes_to_hex,
    (StringArray DataType::Utf8),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let hex = hex::encode(data);
        let result = hex.trim_start_matches('0');

        if !result.is_empty() {
            Some(format!("0x{}", result))
        } else {
            Some("0x0".to_string())
        }
    }
);

udf!(
    hex_to_bytes,
    (BinaryArray DataType::Binary),
    [
        0 => data: (StringArray DataType::Utf8),
    ],
    |data: &str| {
        let data = data.trim_start_matches("0x");
        let data = hex::decode(data).ok()?;

        Some(data)
    }
);

udf!(
    u256_from_str,
    (BinaryArray DataType::Binary),
    [
        0 => data: (StringArray DataType::Utf8),
    ],
    |data: &str| {
        let value = u256::from_str_prefixed(data).ok()?;

        Some(uint256::to_slice(value))
    }
);

udf!(
    u256_eq,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a: &[u8], b: &[u8]| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        Some(a == b)
    }
);

udf!(
    u256_gt,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        Some(a > b)
    }
);

udf!(
    u256_ge,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        Some(a >= b)
    }
);

udf!(
    u256_lt,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        Some(a < b)
    }
);

udf!(
    u256_le,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        Some(a <= b)
    }
);

udf!(
    u256_add,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        let (result, _) = a.overflowing_add(b);

        Some(uint256::to_slice(result))
    }
);

udf!(
    u256_sub,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        let (result, _) = a.overflowing_sub(b);

        Some(uint256::to_slice(result))
    }
);

udf!(
    u256_mul,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        let (result, _) = a.overflowing_mul(b);

        Some(uint256::to_slice(result))
    }
);

udf!(
    u256_div,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = uint256::from_slice(b)?;

        a.checked_div(b).map(uint256::to_slice)
    }
);

udf!(
    u256_pow,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (Int64Array DataType::Int64),
    ],
    |a, b| {
        let a = uint256::from_slice(a)?;
        let b = b as u32;

        let (result, _) = a.overflowing_pow(b);

        Some(uint256::to_slice(result))
    }
);

udf!(
    i256_from_str,
    (BinaryArray DataType::Binary),
    [
        0 => data: (StringArray DataType::Utf8),
    ],
    |data: &str| {
        let value = match i256::from_str_prefixed(data) {
            Ok(value) => value,
            Err(err) => {
                dbg!(err);

                i256::from_str_prefixed("0").unwrap()
            }
        };


        Some(int256::to_slice(value))
    }
);

udf!(
    i256_eq,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        Some(a == b)
    }
);

udf!(
    i256_gt,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        Some(a > b)
    }
);

udf!(
    i256_ge,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        Some(a >= b)
    }
);

udf!(
    i256_lt,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        Some(a < b)
    }
);

udf!(
    i256_le,
    (BooleanArray DataType::Boolean),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        Some(a <= b)
    }
);

udf!(
    i256_add,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        let (result, _) = a.overflowing_add(b);

        Some(int256::to_slice(result))
    }
);

udf!(
    i256_sub,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        let (result, _) = a.overflowing_sub(b);

        Some(int256::to_slice(result))
    }
);

udf!(
    i256_mul,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        let (result, _) = a.overflowing_mul(b);

        Some(int256::to_slice(result))
    }
);

udf!(
    i256_div,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (BinaryArray DataType::Binary),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = int256::from_slice(b)?;

        a.checked_div(b).map(int256::to_slice)
    }
);

udf!(
    i256_pow,
    (BinaryArray DataType::Binary),
    [
        0 => a: (BinaryArray DataType::Binary),
        1 => b: (Int64Array DataType::Int64),
    ],
    |a, b| {
        let a = int256::from_slice(a)?;
        let b = b as u32;

        let (result, _) = a.overflowing_pow(b);

        Some(int256::to_slice(result))
    }
);
