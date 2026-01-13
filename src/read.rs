use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::de::DeserializeOwned;

use crate::{Error, backend, types::DataFormat};

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

pub fn read_record_from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Error> {
    let (data_format, rdr) = open_file(path)?;
    read_record_from_reader(rdr, data_format)
}

pub fn read_records_from_file<T: DeserializeOwned>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, Error> {
    let (data_format, rdr) = open_file(path)?;
    read_records_from_reader(rdr, data_format)
}
