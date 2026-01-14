use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::de::DeserializeOwned;

use crate::{Error, backend, types::DataFormat};

/// Reads a single record from a reader and deserializes it into the specified type.
///
/// This function supports formats that can represent a single record.
///
/// # Supported Formats
///
/// - JSON (always available)
/// - YAML (requires `yaml` feature)
/// - MessagePack (requires `messagepack` feature)
/// - TOML (requires `toml` feature)
///
/// # Errors
///
/// Returns an error if the data format is not supported for single records,
/// or if deserialization fails.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
///
/// use serde::{Deserialize, Serialize};
/// use serdeio::{read_record_from_reader, DataFormat};
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let json_data = r#"{"name": "Alice", "age": 30}"#;
/// let reader = Cursor::new(json_data);
/// let user: User = read_record_from_reader(reader, DataFormat::Json).unwrap();
/// assert_eq!(user.name, "Alice");
/// ```
pub fn read_record_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    data_format: DataFormat,
) -> Result<T, Error> {
    match data_format {
        DataFormat::Auto => Err(Error::AutoNotSupported),
        DataFormat::Json => backend::json::read(reader),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::read(reader),
        #[cfg(feature = "messagepack")]
        DataFormat::MessagePack => backend::messagepack::read(reader),
        #[cfg(feature = "toml")]
        DataFormat::Toml => backend::toml::read(reader),
        _ => Err(Error::UnsupportedFormat(data_format)),
    }
}

/// Reads multiple records from a reader and deserializes them into a vector of the specified type.
///
/// This function supports formats that can represent multiple records.
///
/// # Supported Formats
///
/// - JSON (always available, as an array)
/// - JSON Lines (always available)
/// - CSV (requires `csv` feature)
/// - YAML (requires `yaml` feature, as an array)
/// - MessagePack (requires `messagepack` feature, as an array)
///
/// # Errors
///
/// Returns an error if the data format is not supported for multiple records,
/// or if deserialization fails.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
///
/// use serde::{Deserialize, Serialize};
/// use serdeio::{read_records_from_reader, DataFormat};
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let json_data = r#"[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]"#;
/// let reader = Cursor::new(json_data);
/// let users: Vec<User> = read_records_from_reader(reader, DataFormat::Json).unwrap();
/// assert_eq!(users.len(), 2);
/// ```
pub fn read_records_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    data_format: DataFormat,
) -> Result<Vec<T>, Error> {
    match data_format {
        DataFormat::Auto => Err(Error::AutoNotSupported),
        DataFormat::Json => backend::json::read(reader),
        DataFormat::JsonLines => backend::jsonlines::read(reader),
        #[cfg(feature = "csv")]
        DataFormat::Csv => backend::csv::read(reader),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::read(reader),
        #[cfg(feature = "messagepack")]
        DataFormat::MessagePack => backend::messagepack::read(reader),
        #[allow(unreachable_patterns)]
        _ => Err(Error::UnsupportedFormat(data_format)),
    }
}

/// Reads a single record from a file and deserializes it into the specified type.
///
/// The data format is automatically inferred from the file extension.
///
/// # Supported Formats
///
/// - JSON (.json)
/// - YAML (.yaml, .yml) - requires `yaml` feature
/// - MessagePack (.msgpack, .mpack, .mpk) - requires `messagepack` feature
/// - TOML (.toml) - requires `toml` feature
///
/// # Errors
///
/// Returns an error if the file cannot be opened, the extension is unknown,
/// or deserialization fails.
///
/// # Examples
///
/// ```rust,no_run
/// use serde::{Deserialize, Serialize};
/// use serdeio::{read_record_from_file, DataFormat};
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user: User = read_record_from_file("user.json", DataFormat::Auto).unwrap();
/// ```
pub fn read_record_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    data_format: DataFormat,
) -> Result<T, Error> {
    let path = path.as_ref();
    let final_format = if data_format == DataFormat::Auto {
        DataFormat::try_from(path)?
    } else {
        data_format
    };
    let file = File::open(path)?;
    let rdr = BufReader::new(file);
    read_record_from_reader(rdr, final_format)
}

/// Reads multiple records from a file and deserializes them into a vector of the specified type.
///
/// The data format is automatically inferred from the file extension.
///
/// # Supported Formats
///
/// - JSON (.json)
/// - JSON Lines (.jsonl, .jsl)
/// - CSV (.csv) - requires `csv` feature
/// - YAML (.yaml, .yml) - requires `yaml` feature
/// - MessagePack (.msgpack, .mpack, .mpk) - requires `messagepack` feature
///
/// # Errors
///
/// Returns an error if the file cannot be opened, the extension is unknown,
/// or deserialization fails.
///
/// # Examples
///
/// ```rust,no_run
/// use serde::{Deserialize, Serialize};
/// use serdeio::{read_records_from_file, DataFormat};
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let users: Vec<User> = read_records_from_file("users.json", DataFormat::Auto).unwrap();
/// ```
pub fn read_records_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    data_format: DataFormat,
) -> Result<Vec<T>, Error> {
    let path = path.as_ref();
    let final_format = if data_format == DataFormat::Auto {
        DataFormat::try_from(path)?
    } else {
        data_format
    };
    let file = File::open(path)?;
    let rdr = BufReader::new(file);
    read_records_from_reader(rdr, final_format)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde::Deserialize;

    use super::*;

    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct TestRecord {
        name: String,
        value: i32,
    }

    #[test]
    fn test_read_record_from_reader_auto_not_supported() {
        let json_data = r#"{"name": "test", "value": 42}"#;
        let reader = Cursor::new(json_data);
        let result: Result<TestRecord, Error> = read_record_from_reader(reader, DataFormat::Auto);
        assert!(matches!(result, Err(Error::AutoNotSupported)));
    }

    #[test]
    fn test_read_records_from_reader_auto_not_supported() {
        let json_data = r#"[{"name": "test1", "value": 1}, {"name": "test2", "value": 2}]"#;
        let reader = Cursor::new(json_data);
        let result: Result<Vec<TestRecord>, Error> =
            read_records_from_reader(reader, DataFormat::Auto);
        assert!(matches!(result, Err(Error::AutoNotSupported)));
    }
}
