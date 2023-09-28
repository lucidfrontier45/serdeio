use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use serde::de::DeserializeOwned;

use crate::common::{FileFormat, Result};

fn read_jsonlines<T: DeserializeOwned>(reader: impl Read) -> Result<Vec<T>> {
    let mut records: Vec<T> = Vec::new();
    for line in BufReader::new(reader).lines() {
        let line = line?;
        let record: T = serde_json::from_str(&line)?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_record_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    file_format: FileFormat,
) -> Result<T> {
    match file_format {
        FileFormat::Json => serde_json::from_reader(reader).map_err(|e| e.into()),
        _ => Err(format!("Unsupported file format: {}", file_format).into()),
    }
}

pub fn read_records_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    file_format: FileFormat,
) -> Result<Vec<T>> {
    match file_format {
        FileFormat::Json => serde_json::from_reader(reader).map_err(|e| e.into()),
        FileFormat::JsonLines => read_jsonlines(reader),
    }
}

pub fn read_record_from_path<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
) -> Result<T> {
    let file = File::open(path)?;
    read_record_from_reader(file, file_format)
}

pub fn read_records_from_path<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
) -> Result<Vec<T>> {
    let file = File::open(path)?;
    read_records_from_reader(file, file_format)
}
