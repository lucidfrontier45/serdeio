use std::io::{BufReader, BufWriter, Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let mut reader = BufReader::new(reader);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(toml::from_str(&content)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let mut writer = BufWriter::new(writer);
    let content = toml::to_string(record)?;
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use serde::{Deserialize, Serialize};

    use super::{read, write};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Config {
        name: String,
        version: u32,
        enabled: bool,
    }

    #[test]
    fn test_read_write() {
        let config = Config {
            name: "myapp".to_string(),
            version: 1,
            enabled: true,
        };

        let mut buffer = Vec::new();
        write(&mut buffer, &config).unwrap();

        let reader = Cursor::new(buffer);
        let deserialized: Config = read(reader).unwrap();

        assert_eq!(config, deserialized);
    }
}
