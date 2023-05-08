use std::sync::Arc;

use datafusion::{
    arrow::{
        array::{ArrayRef, BinaryArray, BooleanArray, StringArray, UInt64Array},
        datatypes::DataType,
    },
    common::cast::{as_binary_array, as_string_array},
    logical_expr::{create_udf, ScalarUDF, Volatility},
    physical_plan::functions::make_scalar_function,
};

use crate::blockchain_data::value::Value;

macro_rules! define_as_ty_udf {
    ($f:ident, $ret:expr, $arr:ty, $code:tt) => {
        pub(crate) fn $f() -> ScalarUDF {
            let fun = make_scalar_function(|args: &[ArrayRef]| {
                let arg1 =
                    as_binary_array(&args[0]).expect("BUG: Failed to cast DataFusion value in UDF");

                let array = arg1
                    .iter()
                    .map(|data| match data {
                        Some(data) => $code(data),
                        _ => None,
                    })
                    .collect::<$arr>();

                Ok(Arc::new(array) as ArrayRef)
            });

            create_udf(
                stringify!($f),
                vec![DataType::Binary],
                Arc::new($ret),
                Volatility::Immutable,
                fun,
            )
        }
    };
}

define_as_ty_udf!(
    as_uint64,
    DataType::UInt64,
    UInt64Array,
    (|data| {
        let value = Value::from_slice(data).ok()?;

        value.as_u64()
    })
);

define_as_ty_udf!(
    as_boolean,
    DataType::Boolean,
    BooleanArray,
    (|data| {
        let value = Value::from_slice(data).ok()?;

        value.as_bool()
    })
);

define_as_ty_udf!(
    as_string,
    DataType::Utf8,
    StringArray,
    (|data| {
        let value = Value::from_slice(data).ok()?;

        value.into_string()
    })
);

pub(crate) fn struct_field() -> ScalarUDF {
    let fun = make_scalar_function(|args: &[ArrayRef]| {
        let arg1 = as_binary_array(&args[0]).expect("BUG: Failed to cast DataFusion value in UDF");
        let arg2 = as_string_array(&args[1]).expect("BUG: Failed to cast DataFusion value in UDF");

        let array = arg1
            .iter()
            .zip(arg2)
            .map(|(data, field)| match (data, field) {
                (Some(data), Some(field)) => {
                    let value = Value::from_slice(data).ok()?;
                    let field = &value.as_struct()?.fields.get(field)?;

                    Some(field.serialize())
                }
                _ => None,
            })
            .collect::<BinaryArray>();

        Ok(Arc::new(array) as ArrayRef)
    });

    create_udf(
        "struct_field",
        vec![DataType::Binary, DataType::Utf8],
        Arc::new(DataType::Binary),
        Volatility::Immutable,
        fun,
    )
}

pub(crate) fn bytes_to_hex() -> ScalarUDF {
    let fun = make_scalar_function(|args: &[ArrayRef]| {
        let values =
            as_binary_array(&args[0]).expect("BUG: Failed to cast DataFusion value in UDF");

        let results = values
            .iter()
            .map(|value| value.map(|value| format!("0x{}", hex::encode(value))))
            .collect::<StringArray>();

        Ok(Arc::new(results) as ArrayRef)
    });

    create_udf(
        "bytes_to_hex",
        vec![DataType::Binary],
        Arc::new(DataType::Utf8),
        Volatility::Immutable,
        fun,
    )
}
