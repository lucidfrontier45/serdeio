use std::io::{Read, Write};

use serde::{de::DeserializeOwned, Serialize};

use crate::common::Result;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T> {
    serde_json::from_reader(reader).map_err(|e| e.into())
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<()> {
    serde_json::to_writer(writer, record).map_err(|e| e.into())
}
