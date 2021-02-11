pub mod data_type;

use crate::data_type::DataType;
use serde::de::{IntoDeserializer, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::io::Read;

pub fn read_protocol<R: Read>(reader: &mut R) -> serde_json::Result<Protocol> {
    serde_json::from_reader(reader)
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Protocol {
    pub types: HashMap<String, ProtocolType>,
    #[serde(flatten)]
    pub namespaces: HashMap<String, Namespace>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Namespace {
    Map(HashMap<String, Namespace>),
    DataType(DataType),
}

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
            _ => {
                let data_type = DataType::deserialize(value.into_deserializer())?;
                Ok(ProtocolType::DataType(data_type))
            }
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
    use crate::{read_protocol, ProtocolType};
    use serde_test::{assert_de_tokens, Token};
    use std::fs::File;

    #[test]
    fn test_diablo2_protocol_read() {
        let mut file = File::open("test/diablo2.json").expect("Failed to read file");
        let protocol = read_protocol(&mut file).expect("Failed to read protocol");

        println!("{:#?}", protocol)
    }

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
