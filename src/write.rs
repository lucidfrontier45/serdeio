use std::{fs::File, io::Write, path::Path};

use serde::Serialize;

use crate::{
    backend,
    common::{DataFormat, Result},
};

pub fn write_record_to_writer<T: Serialize>(
    writer: impl Write,
    data_format: DataFormat,
    record: &T,
) -> Result<()> {
    match data_format {
        DataFormat::Json => backend::json::write(writer, record),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::write(writer, record),
        _ => Err(format!("Unsupported file format: {}", data_format).into()),
    }
}

pub fn write_records_to_writer<T: Serialize>(
    writer: impl Write,
    data_format: DataFormat,
    records: &Vec<T>,
) -> Result<()> {
    match data_format {
        DataFormat::Json => backend::json::write(writer, records),
        DataFormat::JsonLines => backend::jsonlines::write(writer, records),
        #[cfg(feature = "csv")]
        DataFormat::Csv => backend::csv::write(writer, records),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::write(writer, records),
    }
}

pub fn write_record_to_file<T: Serialize>(path: impl AsRef<Path>, records: &T) -> Result<()> {
    let data_format = DataFormat::try_from(path.as_ref())?;
    let file = File::create(path)?;
    write_record_to_writer(file, data_format, records)
}

pub fn write_records_to_file<T: Serialize>(path: impl AsRef<Path>, records: &Vec<T>) -> Result<()> {
    let data_format = DataFormat::try_from(path.as_ref())?;
    let file = File::create(path)?;
    write_records_to_writer(file, data_format, records)
}
