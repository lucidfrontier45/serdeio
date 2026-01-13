use std::io::{BufReader, BufWriter, Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let reader = BufReader::new(reader);
    Ok(rmp_serde::decode::from_read(reader)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let mut writer = BufWriter::new(writer);
    rmp_serde::encode::write(&mut writer, record)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use serde::{Deserialize, Serialize};

    use super::{read, write};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct User {
        name: String,
        age: u8,
    }

    #[test]
    fn test_single_user() {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
        };

        let mut buffer = Vec::new();
        write(&mut buffer, &user).unwrap();

        let mut reader = Cursor::new(buffer);
        let deserialized: User = read(&mut reader).unwrap();

        assert_eq!(user, deserialized);
    }

    #[test]
    fn test_multiple_users() {
        let users = vec![
            User {
                name: "Alice".to_string(),
                age: 30,
            },
            User {
                name: "Bob".to_string(),
                age: 25,
            },
        ];

        let mut buffer = Vec::new();
        write(&mut buffer, &users).unwrap();

        let mut reader = Cursor::new(buffer);
        let deserialized: Vec<User> = read(&mut reader).unwrap();

        assert_eq!(users, deserialized);
    }
}
