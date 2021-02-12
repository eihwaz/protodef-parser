pub mod data_type;

use crate::data_type::DataType;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use std::io::Read;

pub fn read_protocol<R: Read>(reader: R) -> serde_json::Result<Protocol> {
    serde_json::from_reader(reader)
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Protocol {
    pub types: LinkedHashMap<String, DataType>,
    #[serde(flatten)]
    pub namespaces: LinkedHashMap<String, Namespace>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Namespace {
    Map(LinkedHashMap<String, Namespace>),
    DataType(DataType),
}

#[cfg(test)]
mod tests {
    use crate::read_protocol;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_decode_protocols_data() {
        let paths = fs::read_dir("test").expect("Failed to open test folder");

        for entry_res in paths.into_iter() {
            let entry = entry_res.expect("Failed to get test folder entry");
            let file = File::open(entry.path()).expect("Failed to read file");

            let name = entry
                .file_name()
                .into_string()
                .expect("Failed to get entry name");

            read_protocol(&file).expect(&format!("Failed to read \"{}\" protocol", name));
        }
    }
}
