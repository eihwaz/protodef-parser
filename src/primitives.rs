use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Primitives {
    Boolean,
    String,
    Void,
}

struct PrimitivesVisitor;

impl<'de> Visitor<'de> for PrimitivesVisitor {
    type Value = Primitives;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid primitive type string")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.as_str() {
            "bool" => Ok(Primitives::Boolean),
            "cstring" => Ok(Primitives::String),
            "void" => Ok(Primitives::Void),
            _ => Err(de::Error::invalid_value(Unexpected::Str(&value), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Primitives {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PrimitivesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Primitives;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_decode_boolean() {
        assert_de_tokens(&Primitives::Boolean, &[Token::String("bool")]);
    }

    #[test]
    fn test_decode_string() {
        assert_de_tokens(&Primitives::String, &[Token::String("cstring")]);
    }

    #[test]
    fn test_decode_void() {
        assert_de_tokens(&Primitives::Void, &[Token::String("void")]);
    }
}
