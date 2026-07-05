use std::io::{BufReader, BufWriter, Read, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let reader = BufReader::new(reader);
    Ok(serde_json::from_reader(reader)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let mut writer = BufWriter::new(writer);
    serde_json::to_writer(&mut writer, record)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::{self, Write};

    use serde::Serialize;

    use super::write;
    use crate::Error;

    /// Writer that succeeds on every write but fails on flush, so a
    /// correct implementation must surface the flush error instead of
    /// silently swallowing it during drop.
    struct FlushFailingWriter {
        inner: Vec<u8>,
    }

    impl Write for FlushFailingWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("flush failed"))
        }
    }

    #[derive(Serialize)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_write_flush_error_propagates() {
        let writer = FlushFailingWriter { inner: Vec::new() };
        let result = write(writer, &Point { x: 1, y: 2 });
        assert!(
            matches!(result, Err(Error::Io(_))),
            "expected flush error to propagate, got {result:?}"
        );
    }
}
