use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Numeric {
    Byte { signed: bool },
    Short { signed: bool, byte_order: ByteOrder },
    Int { signed: bool, byte_order: ByteOrder },
    Long { signed: bool, byte_order: ByteOrder },
    Float { byte_order: ByteOrder },
    Double { byte_order: ByteOrder },
    VarInt,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

struct NumericVisitor;

impl<'de> Visitor<'de> for NumericVisitor {
    type Value = Numeric;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid numeric type string")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.as_str() {
            "i8" => Ok(Numeric::Byte { signed: true }),
            "u8" => Ok(Numeric::Byte { signed: false }),
            "i16" => Ok(Numeric::Short {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            }),
            "u16" => Ok(Numeric::Short {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            }),
            "li16" => Ok(Numeric::Short {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            }),
            "lu16" => Ok(Numeric::Short {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            }),
            "i32" => Ok(Numeric::Int {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            }),
            "u32" => Ok(Numeric::Int {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            }),
            "li32" => Ok(Numeric::Int {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            }),
            "lu32" => Ok(Numeric::Int {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            }),
            "i64" => Ok(Numeric::Long {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            }),
            "u64" => Ok(Numeric::Long {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            }),
            "li64" => Ok(Numeric::Long {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            }),
            "lu64" => Ok(Numeric::Long {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            }),
            "f32" => Ok(Numeric::Float {
                byte_order: ByteOrder::BigEndian,
            }),
            "lf32" => Ok(Numeric::Float {
                byte_order: ByteOrder::LittleEndian,
            }),
            "f64" => Ok(Numeric::Double {
                byte_order: ByteOrder::BigEndian,
            }),
            "lf64" => Ok(Numeric::Double {
                byte_order: ByteOrder::LittleEndian,
            }),
            "varint" => Ok(Numeric::VarInt),
            _ => Err(de::Error::invalid_value(Unexpected::Str(&value), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Numeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NumericVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::numeric::ByteOrder;
    use crate::Numeric;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_decode_i8() {
        assert_de_tokens(&Numeric::Byte { signed: true }, &[Token::String("i8")]);
    }

    #[test]
    fn test_decode_u8() {
        assert_de_tokens(&Numeric::Byte { signed: false }, &[Token::String("u8")]);
    }

    #[test]
    fn test_decode_i16() {
        assert_de_tokens(
            &Numeric::Short {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("i16")],
        );
    }

    #[test]
    fn test_decode_u16() {
        assert_de_tokens(
            &Numeric::Short {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("u16")],
        );
    }

    #[test]
    fn test_decode_li16() {
        assert_de_tokens(
            &Numeric::Short {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("li16")],
        );
    }

    #[test]
    fn test_decode_lu16() {
        assert_de_tokens(
            &Numeric::Short {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lu16")],
        );
    }

    #[test]
    fn test_decode_i32() {
        assert_de_tokens(
            &Numeric::Int {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("i32")],
        );
    }

    #[test]
    fn test_decode_u32() {
        assert_de_tokens(
            &Numeric::Int {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("u32")],
        );
    }

    #[test]
    fn test_decode_li32() {
        assert_de_tokens(
            &Numeric::Int {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("li32")],
        );
    }

    #[test]
    fn test_decode_lu32() {
        assert_de_tokens(
            &Numeric::Int {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lu32")],
        );
    }

    #[test]
    fn test_decode_i64() {
        assert_de_tokens(
            &Numeric::Long {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("i64")],
        );
    }

    #[test]
    fn test_decode_u64() {
        assert_de_tokens(
            &Numeric::Long {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("u64")],
        );
    }

    #[test]
    fn test_decode_li64() {
        assert_de_tokens(
            &Numeric::Long {
                signed: true,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("li64")],
        );
    }

    #[test]
    fn test_decode_lu64() {
        assert_de_tokens(
            &Numeric::Long {
                signed: false,
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lu64")],
        );
    }

    #[test]
    fn test_decode_f32() {
        assert_de_tokens(
            &Numeric::Float {
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("f32")],
        );
    }

    #[test]
    fn test_decode_lf32() {
        assert_de_tokens(
            &Numeric::Float {
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lf32")],
        );
    }

    #[test]
    fn test_decode_f64() {
        assert_de_tokens(
            &Numeric::Double {
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("f64")],
        );
    }

    #[test]
    fn test_decode_lf64() {
        assert_de_tokens(
            &Numeric::Double {
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lf64")],
        );
    }
}
