use std::io::{Read, Write};

use anyhow::Result as AnyResult;
use serde::{Serialize, de::DeserializeOwned};

pub fn read<T: DeserializeOwned>(reader: impl Read) -> AnyResult<T> {
    serde_yaml::from_reader(reader).map_err(|e| e.into())
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> AnyResult<()> {
    serde_yaml::to_writer(writer, record).map_err(|e| e.into())
}
