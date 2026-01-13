use std::io::{BufReader, BufWriter, Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let reader = BufReader::new(reader);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let writer = BufWriter::new(writer);
    Ok(serde_yaml::to_writer(writer, record)?)
}
