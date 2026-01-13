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

fn open_file(path: impl AsRef<Path>) -> Result<(DataFormat, BufReader<File>), Error> {
    let data_format = DataFormat::try_from(path.as_ref())?;
    let file = File::open(path)?;
    let rdr = BufReader::new(file);
    Ok((data_format, rdr))
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
/// use serdeio::read_record_from_file;
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user: User = read_record_from_file("user.json").unwrap();
/// ```
pub fn read_record_from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Error> {
    let (data_format, rdr) = open_file(path)?;
    read_record_from_reader(rdr, data_format)
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
/// use serdeio::read_records_from_file;
///
/// #[derive(Deserialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let users: Vec<User> = read_records_from_file("users.json").unwrap();
/// ```
pub fn read_records_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, Error> {
    let (data_format, rdr) = open_file(path)?;
    read_records_from_reader(rdr, data_format)
}
