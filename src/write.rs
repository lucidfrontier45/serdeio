use std::{fs::File, io::Write, path::Path};

use serde::Serialize;

use crate::{
    backend,
    common::{FileFormat, Result},
};

pub fn write_record_to_writer<T: Serialize>(
    writer: impl Write,
    file_format: FileFormat,
    record: &T,
) -> Result<()> {
    match file_format {
        FileFormat::Json => backend::json::write(writer, record),
        _ => Err(format!("Unsupported file format: {}", file_format).into()),
    }
}

pub fn write_record_to_file<T: Serialize>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
    records: &T,
) -> Result<()> {
    let file = File::create(path)?;
    write_record_to_writer(file, file_format, records)
}

pub fn write_records_to_writer<T: Serialize>(
    writer: impl Write,
    file_format: FileFormat,
    records: &Vec<T>,
) -> Result<()> {
    match file_format {
        FileFormat::Json => backend::json::write(writer, records),
        FileFormat::JsonLines => backend::jsonlines::write(writer, records),
    }
}

pub fn write_records_to_file<T: Serialize>(
    path: impl AsRef<Path>,
    file_format: FileFormat,
    records: &Vec<T>,
) -> Result<()> {
    let file = File::create(path)?;
    write_records_to_writer(file, file_format, records)
}
