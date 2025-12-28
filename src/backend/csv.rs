use std::io::{Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<Vec<T>, Error> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut records: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn write<'a, T: Serialize + 'a>(
    writer: impl Write,
    records: impl IntoIterator<Item = &'a T>,
) -> Result<(), Error> {
    let mut wtr = csv::Writer::from_writer(writer);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use serde::{Deserialize, Serialize};

    use super::{read, write};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Record {
        id: u32,
        name: String,
    }

    #[test]
    fn test_read() {
        let text = r#"
id,name
1,foo
2,bar"#;

        let records: Vec<Record> = read(Cursor::new(text)).unwrap();
        let expected = vec![
            Record {
                id: 1,
                name: "foo".to_owned(),
            },
            Record {
                id: 2,
                name: "bar".to_owned(),
            },
        ];
        assert_eq!(expected, records);
    }

    #[test]
    fn test_write() {
        let records = vec![
            Record {
                id: 1,
                name: "foo".to_owned(),
            },
            Record {
                id: 2,
                name: "bar".to_owned(),
            },
        ];
        let mut cursor = Cursor::new(vec![]);
        write(&mut cursor, &records).unwrap();
        let data = String::from_utf8(cursor.into_inner())
            .unwrap()
            .trim()
            .to_string();
        let expected = r#"
id,name
1,foo
2,bar"#
            .trim()
            .to_owned();
        assert_eq!(expected, data);
    }
}
