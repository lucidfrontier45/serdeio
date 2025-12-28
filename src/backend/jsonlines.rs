use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use anyhow::Result as AnyResult;
use serde::{Serialize, de::DeserializeOwned};

pub fn read<T: DeserializeOwned>(reader: impl Read) -> AnyResult<Vec<T>> {
    let reader = BufReader::new(reader);
    let mut records: Vec<T> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let record: T = serde_json::from_str(&line)?;
        records.push(record);
    }
    Ok(records)
}

pub fn write<'a, T: Serialize + 'a>(
    writer: impl Write,
    records: impl IntoIterator<Item = &'a T>,
) -> AnyResult<()> {
    let mut writer = BufWriter::new(writer);
    for record in records {
        let line = serde_json::to_string(record)?;
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }
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
        items: Vec<String>,
    }

    #[test]
    fn test_read() {
        let data = r#"
            {"id": 1, "name": "foo", "items": ["a", "b", "c"]}
            {"id": 2, "name": "bar", "items": ["d", "e", "f"]}
        "#
        .trim();
        let cursor = Cursor::new(data);
        let records: Vec<Record> = read(cursor).unwrap();

        let expected = vec![
            Record {
                id: 1,
                name: "foo".to_owned(),
                items: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            },
            Record {
                id: 2,
                name: "bar".to_owned(),
                items: vec!["d".to_owned(), "e".to_owned(), "f".to_owned()],
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
                items: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            },
            Record {
                id: 2,
                name: "bar".to_owned(),
                items: vec!["d".to_owned(), "e".to_owned(), "f".to_owned()],
            },
        ];
        let mut cursor = Cursor::new(Vec::new());
        write(&mut cursor, &records).unwrap();
        let data = String::from_utf8(cursor.into_inner())
            .unwrap()
            .trim()
            .to_string();
        let expected = r#"
{"id":1,"name":"foo","items":["a","b","c"]}
{"id":2,"name":"bar","items":["d","e","f"]}"#
            .trim();
        assert_eq!(expected, data);
    }
}
