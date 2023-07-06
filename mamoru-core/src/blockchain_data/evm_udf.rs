use std::sync::Arc;

use datafusion::arrow::array::{BinaryArray, BooleanArray, Int64Array, StringArray};
use datafusion::arrow::datatypes::DataType;
use ethabi::Token;
use tracing::warn;

use crate::blockchain_data::evm_value::{parse_evm_tx_input, TxInput};
use crate::{deserialize_data, serialize_data, udf};

udf!(
    evm_parse_tx_input,
    (BinaryArray DataType::Binary),
    [
        0 => input: (BinaryArray DataType::Binary),
        1 => abi: (StringArray DataType::Utf8),
    ],
    |input, abi| {
        match parse_evm_tx_input(abi, input) {
            Ok(result) => result.map(|input| serialize_data(&input.0)),
            Err(err) => {
                warn!("Error while parsing EVM input: {}", err);

                None
            }
        }
    }
);

udf!(
    evm_take_token,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
        1 => index: (Int64Array DataType::Int64),
    ],
    |data, index| {
        let input = TxInput::from_slice(data).ok()?;

        input.tokens().get(index as usize).map(|token| {
            serialize_data(token)
        })
    }
);

udf!(
    evm_as_boolean,
    (BooleanArray DataType::Boolean),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_bool()
    }
);

udf!(
    evm_as_address,
    (StringArray DataType::Utf8),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_address().map(|t| format!("{t:#x}"))
    }
);

udf!(
    evm_as_uint256,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_uint().map(|t| {
            let parts: [u8; 32] = t.into();

            // Store as big endian bytes
            parts
        })
    }
);

udf!(
    evm_as_int256,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_int().map(|t| {
            let parts: [u8; 32] = t.into();

            // Store as big endian bytes
            parts
        })
    }
);

udf!(
    evm_as_string,
    (StringArray DataType::Utf8),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_string()
    }
);

udf!(
    evm_as_bytes,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_bytes()
    }
);

udf!(
    evm_as_fixed_bytes,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_fixed_bytes()
    }
);

udf!(
    evm_as_array,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_array().map(|tokens| serialize_data(&tokens))
    }
);

udf!(
    evm_as_fixed_array,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_fixed_array().map(|tokens| serialize_data(&tokens))
    }
);

udf!(
    evm_as_tuple,
    (BinaryArray DataType::Binary),
    [
        0 => data: (BinaryArray DataType::Binary),
    ],
    |data| {
        let token = deserialize_data::<Token>(data).ok()?;

        token.into_tuple().map(|tokens| serialize_data(&tokens))
    }
);
