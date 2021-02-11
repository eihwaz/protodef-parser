pub mod data_type;

use crate::data_type::{DataType, DataTypeVisitor};
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum ProtocolType {
    Native,
    DataType(DataType),
}

struct ProtocolTypeVisitor;

impl<'de> Visitor<'de> for ProtocolTypeVisitor {
    type Value = ProtocolType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an \"native\" or valid data type string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "native" => Ok(ProtocolType::Native),
            _ => Ok(ProtocolType::DataType(
                DataTypeVisitor.visit_str::<E>(value)?,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for ProtocolType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ProtocolTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{DataType, Numeric};
    use crate::ProtocolType;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_decode_native_protocol_type() {
        assert_de_tokens(&ProtocolType::Native, &[Token::String("native")]);
    }

    #[test]
    fn test_decode_varint_data_type_protocol_type() {
        assert_de_tokens(
            &ProtocolType::DataType(DataType::Numeric(Numeric::VarInt)),
            &[Token::String("varint")],
        );
    }

    #[test]
    fn test_decode_custom_data_type_protocol_type() {
        assert_de_tokens(
            &ProtocolType::DataType(DataType::Custom("optionalNbt".to_string())),
            &[Token::String("optionalNbt")],
        );
    }
}
