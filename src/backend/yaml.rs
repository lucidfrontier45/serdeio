use std::io::{Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    Ok(serde_yaml::to_writer(writer, record)?)
}
