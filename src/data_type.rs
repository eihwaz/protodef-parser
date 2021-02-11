use serde::de;
use serde::de::{IntoDeserializer, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    Numeric(Numeric),
    Primitive(Primitive),
    Structure(Box<Structure>),
    Custom(String),
}

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

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Primitive {
    #[serde(rename = "bool")]
    Boolean,
    #[serde(rename = "cstring")]
    String,
    Void,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Structure {
    /// Represents a list of values with same type.
    Array {
        /// The type of length prefix.
        count_type: DataType,
        /// The type of the elements.
        elements_type: DataType,
    },
    /// Represents a list of named values.
    Container { fields: Vec<Field> },
    /// Represents a count field for an array or a buffer.
    Count {
        /// The type of count
        count_type: DataType,
        /// A field to count for.
        count_for: String,
    },
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: DataType,
}

struct NumericVisitor;

impl<'de> Visitor<'de> for NumericVisitor {
    type Value = Numeric;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid numeric type string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
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

struct StructureVisitor;

impl<'de> Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid primitive structure seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let struct_type: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        match struct_type.as_str() {
            "container" => {
                let fields = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Structure::Container { fields })
            }
            "array" => {
                let mut array_fields: HashMap<String, DataType> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let count_type = array_fields
                    .remove("countType")
                    .ok_or_else(|| de::Error::missing_field("countType"))?;

                let elements_type = array_fields
                    .remove("type")
                    .ok_or_else(|| de::Error::missing_field("type"))?;

                Ok(Structure::Array {
                    count_type,
                    elements_type,
                })
            }
            "count" => {
                let mut count_fields: HashMap<String, String> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let untyped_count_type = count_fields
                    .remove("type")
                    .ok_or_else(|| de::Error::missing_field("type"))?;

                let count_type = DataType::deserialize(untyped_count_type.into_deserializer())?;

                let count_for = count_fields
                    .remove("countFor")
                    .ok_or_else(|| de::Error::missing_field("countFor"))?;

                Ok(Structure::Count {
                    count_type,
                    count_for,
                })
            }
            unknown_variant => Err(de::Error::unknown_variant(
                unknown_variant,
                &["container", "array", "count"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Structure {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StructureVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{ByteOrder, DataType, Field, Numeric, Primitive, Structure};
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
    fn test_decode_f64_numeric() {
        assert_de_tokens(
            &Numeric::Double {
                byte_order: ByteOrder::BigEndian,
            },
            &[Token::String("f64")],
        );
    }

    #[test]
    fn test_decode_lf64_numeric() {
        assert_de_tokens(
            &Numeric::Double {
                byte_order: ByteOrder::LittleEndian,
            },
            &[Token::String("lf64")],
        );
    }

    #[test]
    fn test_decode_boolean() {
        assert_de_tokens(
            &Primitive::Boolean,
            &[
                Token::Enum { name: "Primitive" },
                Token::String("bool"),
                Token::Unit,
            ],
        );
    }

    #[test]
    fn test_decode_string() {
        assert_de_tokens(
            &Primitive::String,
            &[
                Token::Enum { name: "Primitive" },
                Token::String("cstring"),
                Token::Unit,
            ],
        );
    }

    #[test]
    fn test_decode_void() {
        assert_de_tokens(
            &Primitive::Void,
            &[
                Token::Enum { name: "Primitive" },
                Token::String("void"),
                Token::Unit,
            ],
        );
    }

    #[test]
    fn test_decode_numeric_data_type() {
        assert_de_tokens(
            &DataType::Numeric(Numeric::Float {
                byte_order: ByteOrder::BigEndian,
            }),
            &[Token::String("f32")],
        );
    }

    #[test]
    fn test_decode_primitive_data_type() {
        assert_de_tokens(
            &DataType::Primitive(Primitive::Boolean),
            &[Token::String("bool")],
        );
    }

    #[test]
    fn test_decode_container_data_type() {
        let fields = vec![Field {
            name: "serverPort".to_string(),
            field_type: DataType::Numeric(Numeric::Short {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            }),
        }];

        let container = Structure::Container { fields };

        assert_de_tokens(
            &container,
            &[
                Token::Seq { len: Some(2) },
                Token::String("container"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Field",
                    len: 2,
                },
                Token::Str("name"),
                Token::String("serverPort"),
                Token::Str("type"),
                Token::String("u16"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_container_with_container_data_type() {
        let inner_container_fields = vec![Field {
            name: "name".to_string(),
            field_type: DataType::Numeric(Numeric::VarInt),
        }];

        let fields = vec![Field {
            name: "inner_container".to_string(),
            field_type: DataType::Structure(Box::new(Structure::Container {
                fields: inner_container_fields,
            })),
        }];

        let container = Structure::Container { fields };

        assert_de_tokens(
            &container,
            &[
                Token::Seq { len: Some(2) },
                Token::String("container"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Field",
                    len: 2,
                },
                Token::Str("name"),
                Token::String("inner_container"),
                Token::Str("type"),
                Token::Seq { len: Some(2) },
                Token::String("container"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Field",
                    len: 2,
                },
                Token::Str("name"),
                Token::String("name"),
                Token::Str("type"),
                Token::String("varint"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_array_data_type() {
        let array = Structure::Array {
            count_type: DataType::Numeric(Numeric::VarInt),
            elements_type: DataType::Primitive(Primitive::String),
        };

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct { name: "", len: 2 },
                Token::Str("countType"),
                Token::String("varint"),
                Token::Str("type"),
                Token::String("cstring"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_array_with_container_data_type() {
        let fields = vec![Field {
            name: "name".to_string(),
            field_type: DataType::Numeric(Numeric::VarInt),
        }];

        let array = Structure::Array {
            count_type: DataType::Numeric(Numeric::VarInt),
            elements_type: DataType::Structure(Box::new(Structure::Container { fields })),
        };

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct { name: "", len: 2 },
                Token::Str("countType"),
                Token::String("varint"),
                Token::Str("type"),
                Token::Seq { len: Some(2) },
                Token::String("container"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Field",
                    len: 2,
                },
                Token::Str("name"),
                Token::String("name"),
                Token::Str("type"),
                Token::String("varint"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_count_data_type() {
        let count = Structure::Count {
            count_type: DataType::Numeric(Numeric::VarInt),
            count_for: "test".to_string(),
        };

        assert_de_tokens(
            &count,
            &[
                Token::Seq { len: Some(2) },
                Token::String("count"),
                Token::Struct { name: "", len: 2 },
                Token::Str("type"),
                Token::String("varint"),
                Token::Str("countFor"),
                Token::String("test"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }
}
