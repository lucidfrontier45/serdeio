use std::{fs::File, io::Read, path::Path};

use anyhow::{anyhow, Result as AnyResult};
use serde::de::DeserializeOwned;

use crate::{backend, types::DataFormat};

pub fn read_record_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    data_format: DataFormat,
) -> AnyResult<T> {
    match data_format {
        DataFormat::Json => backend::json::read(reader),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::read(reader),
        _ => Err(anyhow!("Unsupported file format: {}", data_format)),
    }
}

pub fn read_records_from_reader<T: DeserializeOwned>(
    reader: impl Read,
    data_format: DataFormat,
) -> AnyResult<Vec<T>> {
    match data_format {
        DataFormat::Json => backend::json::read(reader),
        DataFormat::JsonLines => backend::jsonlines::read(reader),
        #[cfg(feature = "csv")]
        DataFormat::Csv => backend::csv::read(reader),
        #[cfg(feature = "yaml")]
        DataFormat::Yaml => backend::yaml::read(reader),
    }
}

pub fn read_record_from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> AnyResult<T> {
    let data_format = DataFormat::try_from(path.as_ref())?;
    let file = File::open(path)?;
    read_record_from_reader(file, data_format)
}

pub fn read_records_from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> AnyResult<Vec<T>> {
    let data_format = DataFormat::try_from(path.as_ref())?;
    let file = File::open(path)?;
    read_records_from_reader(file, data_format)
}
