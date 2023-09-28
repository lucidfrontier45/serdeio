use std::{fs::File, io::Read, path::Path};

use serde::de::DeserializeOwned;

use crate::{
    backend,
    common::{FileFormat, Result},
};

pub fn read_record_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    file_format: FileFormat,
) -> Result<T> {
    match file_format {
        FileFormat::Json => backend::json::read(reader),
        _ => Err(format!("Unsupported file format: {}", file_format).into()),
    }
}

pub fn read_records_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    file_format: FileFormat,
) -> Result<Vec<T>> {
    match file_format {
        FileFormat::Json => backend::json::read(reader),
        FileFormat::JsonLines => backend::jsonlines::read(reader),
        #[cfg(feature = "csv")]
        FileFormat::Csv => backend::csv::read(reader),
    }
}

pub fn read_record_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
) -> Result<T> {
    let file = File::open(path)?;
    read_record_from_reader(file, file_format)
}

pub fn read_records_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
) -> Result<Vec<T>> {
    let file = File::open(path)?;
    read_records_from_reader(file, file_format)
}
