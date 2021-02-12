use linked_hash_map::LinkedHashMap;
use serde::de;
use serde::de::{SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    Conditional(Box<Conditional>),
    Numeric(Numeric),
    Primitive(Primitive),
    Structure(Box<Structure>),
    Util(Box<Util>),
    Custom(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Conditional {
    Switch(Switch),
    Option(DataType),
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Switch {
    name: Option<String>,
    #[serde(rename = "compareTo")]
    compare_to: String,
    fields: LinkedHashMap<String, DataType>,
    default: Option<DataType>,
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
    Array(Array),
    /// Represents a list of named values.
    Container(Vec<Field>),
    /// Represents a count field for an array or a buffer.
    Count(Count),
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Array {
    /// The type of length prefix.
    #[serde(rename = "countType")]
    pub count_type: Option<DataType>,
    /// A reference to the field counting the elements, or a fixed size.
    pub count: Option<ArrayCount>,
    /// The type of the elements.
    #[serde(rename = "type")]
    pub elements_type: DataType,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ArrayCount {
    /// Reference to the field counting the elements.
    FieldReference(String),
    /// Array with fixed length.
    FixedLength(u32),
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Field {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub field_type: DataType,
    // Useless.
    #[serde(rename = "anon")]
    anonymous: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Count {
    /// The type of count.
    #[serde(rename = "type")]
    pub count_type: DataType,
    /// A field to count for.
    #[serde(rename = "countFor")]
    pub count_for: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Util {
    Buffer(Buffer),
    Mapper(Mapper),
    Bitfield(Vec<BitField>),
    PrefixedString { count_type: DataType },
    Loop(Box<Loop>),
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Buffer {
    /// The type of length prefix.
    #[serde(rename = "countType")]
    pub count_type: Option<DataType>,
    /// A reference to the field counting the elements, or a fixed size.
    pub count: Option<ArrayCount>,
    /// Represent rest bytes as-is.
    pub rest: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Mapper {
    #[serde(rename = "type")]
    pub mappings_type: String,
    pub mappings: LinkedHashMap<String, String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BitField {
    pub name: String,
    pub size: usize,
    pub signed: bool,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Loop {
    #[serde(rename = "endVal")]
    pub end_val: u32,
    #[serde(rename = "type")]
    pub data_type: DataType,
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

struct ConditionalVisitor;

impl<'de> Visitor<'de> for ConditionalVisitor {
    type Value = Conditional;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid `Conditional`")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let conditional_type: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        match conditional_type.as_str() {
            "switch" => {
                let switch = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Conditional::Switch(switch))
            }
            "option" => {
                let data_type = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Conditional::Option(data_type))
            }
            unknown_variant => {
                // Attempt to find switch with wrong type.
                let mut map: HashMap<String, String> = seq.next_element()?.ok_or_else(|| {
                    de::Error::unknown_variant(unknown_variant, &["switch", "option"])
                })?;

                if map.len() == 1 {
                    if let Some(compare_to) = map.remove("compareTo") {
                        let switch = Switch {
                            name: Some(unknown_variant.to_owned()),
                            compare_to,
                            fields: LinkedHashMap::new(),
                            default: None,
                        };

                        return Ok(Conditional::Switch(switch));
                    }
                }

                Err(de::Error::unknown_variant(
                    unknown_variant,
                    &["switch", "option"],
                ))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Conditional {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ConditionalVisitor)
    }
}

struct StructureVisitor;

impl<'de> Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid `Structure`")
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

                Ok(Structure::Container(fields))
            }
            "array" => {
                let array = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Structure::Array(array))
            }
            "count" => {
                let count = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Structure::Count(count))
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

struct UtilVisitor;

impl<'de> Visitor<'de> for UtilVisitor {
    type Value = Util;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an valid util")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let util_type: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        match util_type.as_str() {
            "buffer" => {
                let buffer = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Util::Buffer(buffer))
            }
            "mapper" => {
                let mapper = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Util::Mapper(mapper))
            }
            "bitfield" => {
                let bitfields = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Util::Bitfield(bitfields))
            }
            "pstring" => {
                let mut map: HashMap<String, DataType> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let count_type = map
                    .remove("countType")
                    .ok_or_else(|| de::Error::missing_field("countType"))?;

                Ok(Util::PrefixedString { count_type })
            }
            unknown_variant => {
                // This is what happens when the nodejs developers write a "cool" spec.
                let loop_util: Loop = seq.next_element()?.ok_or_else(|| {
                    de::Error::unknown_variant(
                        unknown_variant,
                        &["buffer", "mapper", "bitfield", "pstring"],
                    )
                })?;

                Ok(Util::Loop(Box::new(loop_util)))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Util {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(UtilVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{
        Array, ArrayCount, Buffer, ByteOrder, Conditional, Count, DataType, Field, Numeric,
        Primitive, Structure, Switch, Util,
    };
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
            name: Some("serverPort".to_string()),
            field_type: DataType::Numeric(Numeric::Short {
                signed: false,
                byte_order: ByteOrder::BigEndian,
            }),
            anonymous: None,
        }];

        let container = Structure::Container(fields);

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
                Token::Some,
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
            name: Some("name".to_string()),
            field_type: DataType::Numeric(Numeric::VarInt),
            anonymous: None,
        }];

        let fields = vec![Field {
            name: Some("inner_container".to_string()),
            field_type: DataType::Structure(Box::new(Structure::Container(inner_container_fields))),
            anonymous: None,
        }];

        let container = Structure::Container(fields);

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
                Token::Some,
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
        let array = Structure::Array(Array {
            count_type: Some(DataType::Numeric(Numeric::VarInt)),
            count: None,
            elements_type: DataType::Primitive(Primitive::String),
        });

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct {
                    name: "Array",
                    len: 2,
                },
                Token::Str("countType"),
                Token::Some,
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
            name: Some("name".to_string()),
            field_type: DataType::Numeric(Numeric::VarInt),
            anonymous: None,
        }];

        let array = Structure::Array(Array {
            count_type: Some(DataType::Numeric(Numeric::VarInt)),
            count: None,
            elements_type: DataType::Structure(Box::new(Structure::Container(fields))),
        });

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct {
                    name: "Array",
                    len: 2,
                },
                Token::Str("countType"),
                Token::Some,
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
        let count = Structure::Count(Count {
            count_type: DataType::Numeric(Numeric::VarInt),
            count_for: "test".to_string(),
        });

        assert_de_tokens(
            &count,
            &[
                Token::Seq { len: Some(2) },
                Token::String("count"),
                Token::Struct {
                    name: "Count",
                    len: 2,
                },
                Token::Str("type"),
                Token::String("varint"),
                Token::Str("countFor"),
                Token::String("test"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_array_ref_field() {
        let array = Structure::Array(Array {
            count_type: None,
            count: Some(ArrayCount::FieldReference("field".to_string())),
            elements_type: DataType::Primitive(Primitive::String),
        });

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct {
                    name: "Array",
                    len: 2,
                },
                Token::Str("count"),
                Token::Some,
                Token::String("field"),
                Token::Str("type"),
                Token::String("cstring"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_array_fixed_length() {
        let array = Structure::Array(Array {
            count_type: None,
            count: Some(ArrayCount::FixedLength(4)),
            elements_type: DataType::Primitive(Primitive::String),
        });

        assert_de_tokens(
            &array,
            &[
                Token::Seq { len: Some(2) },
                Token::String("array"),
                Token::Struct {
                    name: "Array",
                    len: 2,
                },
                Token::Str("count"),
                Token::Some,
                Token::I32(4),
                Token::Str("type"),
                Token::String("cstring"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_prefixed_string() {
        let util = Util::PrefixedString {
            count_type: DataType::Numeric(Numeric::VarInt),
        };

        assert_de_tokens(
            &util,
            &[
                Token::Seq { len: Some(2) },
                Token::Str("pstring"),
                Token::Struct { name: "", len: 1 },
                Token::Str("countType"),
                Token::String("varint"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_buffer() {
        let util = Util::Buffer(Buffer {
            count_type: Some(DataType::Numeric(Numeric::Int {
                signed: true,
                byte_order: ByteOrder::BigEndian,
            })),
            count: None,
            rest: None,
        });

        assert_de_tokens(
            &util,
            &[
                Token::Seq { len: Some(2) },
                Token::String("buffer"),
                Token::Struct {
                    name: "Buffer",
                    len: 1,
                },
                Token::String("countType"),
                Token::Some,
                Token::String("i32"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_option() {
        let conditional = Conditional::Option(DataType::Numeric(Numeric::Byte { signed: false }));

        assert_de_tokens(
            &conditional,
            &[
                Token::Seq { len: Some(2) },
                Token::String("option"),
                Token::String("u8"),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn test_decode_bad_switch() {
        let conditional = Conditional::Switch(Switch {
            name: Some("particleData".to_string()),
            compare_to: "particleId".to_string(),
            fields: Default::default(),
            default: None,
        });

        assert_de_tokens(
            &conditional,
            &[
                Token::Seq { len: Some(2) },
                Token::String("particleData"),
                Token::Struct { name: "", len: 1 },
                Token::String("compareTo"),
                Token::String("particleId"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        );
    }
}
