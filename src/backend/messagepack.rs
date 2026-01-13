use std::io::{Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    rmp_serde::decode::from_read(reader).map_err(Error::from)
}

pub fn write<T: Serialize>(mut writer: impl Write, record: &T) -> Result<(), Error> {
    rmp_serde::encode::write(&mut writer, record).map_err(Error::from)
}
