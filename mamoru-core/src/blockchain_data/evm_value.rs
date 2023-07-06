use ethabi::param_type::Reader;
pub(crate) use ethabi::Token;
use ethabi::{decode, short_signature, ParamType};
pub use ethnum::{i256, u256};

use crate::errors::ParseEvmInputError;
use crate::{deserialize_data, serialize_data};

pub(crate) struct TxInput(pub(crate) Vec<Token>);

impl TxInput {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self(tokens)
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.0
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        serialize_data(&self.0)
    }

    pub(crate) fn from_slice(data: &[u8]) -> Result<Self, ParseEvmInputError> {
        let tokens =
            deserialize_data(data).map_err(|err| ParseEvmInputError::Deserialize(Box::new(err)))?;

        Ok(Self::new(tokens))
    }
}

/// Parses the input data of an EVM transaction into a list of tokens.
/// Returns `None` if the input data does not match the given ABI signature.
pub(crate) fn parse_evm_tx_input(
    abi: &str,
    input: &[u8],
) -> Result<Option<TxInput>, ParseEvmInputError> {
    let (name, parameters) = parse_eth_signature(abi).ok_or_else(|| {
        ParseEvmInputError::ParseABI(format!(
            "Invalid ABI signature: \"{}\". The correct format is \"name(type1,type2)\"",
            abi
        ))
    })?;

    if input.len() < 4 {
        return Err(ParseEvmInputError::InvalidInputData(format!(
            "The function input bytes are too short: {:?}",
            input
        )));
    }

    let parameters_tuple = Reader::read(parameters).map_err(ParseEvmInputError::ReadParameters)?;

    let types: Vec<ParamType> = if let ParamType::Tuple(types) = parameters_tuple {
        types
    } else {
        return Err(ParseEvmInputError::ParseABI(format!(
            "The function ABI is invalid: {:?}",
            input
        )));
    };

    let expected_signature = short_signature(name, &types);
    let actual_signature = &input[..4];

    if expected_signature != actual_signature {
        return Ok(None);
    }

    let data = &input[4..];
    let tokens = decode(&types, data).map_err(ParseEvmInputError::Decode)?;

    Ok(Some(TxInput::new(tokens)))
}

/// Parses a function signature in the format `name(type1,type2)`.
fn parse_eth_signature(signature: &str) -> Option<(&str, &str)> {
    let open_paren_index = signature.find('(')?;
    let close_paren_index = signature.rfind(')')?;

    if open_paren_index >= close_paren_index {
        return None;
    }

    let function_name = signature.trim().get(..open_paren_index)?;
    let parameters_str = &signature[open_paren_index..];

    Some((function_name, parameters_str))
}

pub(crate) mod uint256 {
    use super::u256;

    #[inline]
    pub(crate) fn from_slice(data: &[u8]) -> Option<u256> {
        if data.len() != 32 {
            return None;
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(data);

        Some(u256::from_be_bytes(bytes))
    }

    #[inline]
    pub(crate) fn to_slice(value: u256) -> [u8; 32] {
        value.to_be_bytes()
    }
}

pub(crate) mod int256 {
    use super::i256;

    #[inline]
    pub(crate) fn from_slice(data: &[u8]) -> Option<i256> {
        if data.len() != 32 {
            return None;
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(data);

        Some(i256::from_be_bytes(bytes))
    }

    #[inline]
    pub(crate) fn to_slice(value: i256) -> [u8; 32] {
        value.to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};
    use ethabi::ethereum_types::Address;
    use ethabi::Token;

    use crate::blockchain_data::evm_value::TxInput;

    use super::parse_eth_signature;

    #[test]
    fn parse_eth_signature_ok() {
        assert_eq!(
            parse_eth_signature("test(uint256,bool)"),
            Some(("test", "(uint256,bool)"))
        );
    }

    #[test]
    fn parse_tuple_ok() {
        assert_eq!(
            parse_eth_signature("addIncident(string,(string,string,address,bytes,int32,uint64))"),
            Some((
                "addIncident",
                "(string,(string,string,address,bytes,int32,uint64))"
            ))
        );
    }

    #[test]
    fn parse_eth_signature_no_params() {
        assert_eq!(parse_eth_signature("test()"), Some(("test", "()")));
    }

    #[test]
    fn parse_eth_signature_no_paren() {
        assert_eq!(parse_eth_signature("test"), None);
    }

    #[test]
    fn parse_eth_signature_no_close_paren() {
        assert_eq!(parse_eth_signature("test("), None);
    }

    // cargo test -p mamoru-core evm_value::tests::generate_test_tx_input -- --ignored --nocapture
    #[test]
    #[ignore]
    fn generate_test_tx_input() {
        let input = TxInput::new(vec![
            Token::Address(Address::from([0u8; 20])),
            Token::FixedBytes(vec![1, 2, 3]),
            Token::Bytes(vec![1, 2, 3]),
            Token::Int(1.into()),
            Token::Uint(1.into()),
            Token::Bool(true),
            Token::String("test".to_string()),
            Token::FixedArray(vec![Token::Bool(true), Token::Bool(false)]),
            Token::Array(vec![Token::Bool(true), Token::Bool(false)]),
            Token::Tuple(vec![Token::Bool(true), Token::Bool(false)]),
        ]);

        dbg!(general_purpose::STANDARD.encode(input.serialize()));
    }
}
