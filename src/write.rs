use std::{fs::File, io::Write, path::Path};

use serde::Serialize;

use crate::{Error, backend, types::DataFormat};

/// Writes a single record to a writer in the specified data format.
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
/// or if serialization fails.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
///
/// use serde::{Deserialize, Serialize};
/// use serdeio::{write_record_to_writer, DataFormat};
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user = User { name: "Alice".to_string(), age: 30 };
/// let mut buffer = Vec::new();
/// write_record_to_writer(&mut buffer, &user, DataFormat::Json).unwrap();
/// let json = String::from_utf8(buffer).unwrap();
/// assert!(json.contains("Alice"));
/// ```
pub fn write_record_to_writer<T: Serialize>(
    writer: impl Write,
    record: &T,
    data_format: DataFormat,
) -> Result<(), Error> {
    match data_format {
        DataFormat::Auto => Err(Error::AutoNotSupported),
        DataFormat::Json => backend::json::write(writer, record),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::write(writer, record),
        #[cfg(feature = "messagepack")]
        DataFormat::MessagePack => backend::messagepack::write(writer, record),
        #[cfg(feature = "toml")]
        DataFormat::Toml => backend::toml::write(writer, record),
        _ => Err(Error::UnsupportedFormat(data_format)),
    }
}

/// Writes multiple records to a writer in the specified data format.
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
/// or if serialization fails.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
///
/// use serde::{Deserialize, Serialize};
/// use serdeio::{write_records_to_writer, DataFormat};
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let users = vec![
///     User { name: "Alice".to_string(), age: 30 },
///     User { name: "Bob".to_string(), age: 25 },
/// ];
/// let mut buffer = Vec::new();
/// write_records_to_writer(&mut buffer, &users, DataFormat::Json).unwrap();
/// let json = String::from_utf8(buffer).unwrap();
/// assert!(json.contains("Alice") && json.contains("Bob"));
/// ```
pub fn write_records_to_writer<'a, T: Serialize + 'a>(
    writer: impl Write,
    records: impl IntoIterator<Item = &'a T>,
    data_format: DataFormat,
) -> Result<(), Error> {
    match data_format {
        DataFormat::Auto => Err(Error::AutoNotSupported),
        DataFormat::Json => backend::json::write(writer, &records.into_iter().collect::<Vec<_>>()),
        DataFormat::JsonLines => backend::jsonlines::write(writer, records),
        #[cfg(feature = "csv")]
        DataFormat::Csv => backend::csv::write(writer, records),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::write(writer, &records.into_iter().collect::<Vec<_>>()),
        #[cfg(feature = "messagepack")]
        DataFormat::MessagePack => {
            backend::messagepack::write(writer, &records.into_iter().collect::<Vec<_>>())
        }
        #[allow(unreachable_patterns)]
        _ => Err(Error::UnsupportedFormat(data_format)),
    }
}

/// Writes a single record to a file in the data format inferred from the file extension.
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
/// Returns an error if the file cannot be created, the extension is unknown,
/// or serialization fails.
///
/// # Examples
///
/// ```rust,no_run
/// use serde::{Deserialize, Serialize};
/// use serdeio::{write_record_to_file, DataFormat};
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user = User { name: "Alice".to_string(), age: 30 };
/// write_record_to_file("user.json", &user, DataFormat::Auto).unwrap();
/// ```
pub fn write_record_to_file<T: Serialize>(
    path: impl AsRef<Path>,
    record: &T,
    mut data_format: DataFormat,
) -> Result<(), Error> {
    let inferred_format = DataFormat::try_from(path.as_ref())?;
    if data_format == DataFormat::Auto {
        data_format = inferred_format;
    }
    let file = File::create(path)?;
    write_record_to_writer(file, record, data_format)
}

/// Writes multiple records to a file in the data format inferred from the file extension.
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
/// Returns an error if the file cannot be created, the extension is unknown,
/// or serialization fails.
///
/// # Examples
///
/// ```rust,no_run
/// use serde::{Deserialize, Serialize};
/// use serdeio::{write_records_to_file, DataFormat};
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let users = vec![
///     User { name: "Alice".to_string(), age: 30 },
///     User { name: "Bob".to_string(), age: 25 },
/// ];
/// write_records_to_file("users.json", &users, DataFormat::Auto).unwrap();
/// ```
pub fn write_records_to_file<'a, T: Serialize + 'a, I: IntoIterator<Item = &'a T>>(
    path: impl AsRef<Path>,
    records: I,
    mut data_format: DataFormat,
) -> Result<(), Error> {
    let inferred_format = DataFormat::try_from(path.as_ref())?;
    if data_format == DataFormat::Auto {
        data_format = inferred_format;
    }
    let file = File::create(path)?;
    write_records_to_writer(file, records, data_format)
}
