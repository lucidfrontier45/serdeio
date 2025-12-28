# SerdeIO
Tiny IO utility library for Rust to serialize/deserialize Serde compatible structs

# Install

```sh
cargo add serdeio
```

SerdeIO supports JSON and JSON Lines formats. Additional formats are supported by enabling corresponding features.

- `yaml`
- `csv`

# How to use

- `read_record_from_reader` is used to read a deserializable type `T` from `std::io::Read`. Data format must be specified by `DataFormat` enum.
- `read_records_from_reader` always tries to deserialize the data as `Vec<T>`.
- `read_record_from_file` accepts an `AsRef<Path>`. Data format is automatically determined by file extension.
- `write_*` functions follow the same rules as `read_*`.

Note that some data format like CSV and JSON Lines support only reading records `Vec<T>`.

# Examples

The following code read a JSON file and parse it as `Vec<User>`. Then it encodes the data into YAML format and write it to STDOUT.

```rust
use anyhow::{Context, Result as AnyResult};
use serde::{Deserialize, Serialize};
use serdeio::{DataFormat, read_record_from_file, write_records_to_writer};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
    items: Vec<String>,
}

pub fn main() -> AnyResult<()> {
    // get input file path from argv
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];

    // read json to memory
    let users: Vec<User> =
        read_record_from_file(input_file_path).context("Failed to read records from file")?;

    // write to stdout in json lines format
    let writer = std::io::stdout();
    write_records_to_writer(writer, DataFormat::JsonLines, &users).unwrap();

    Ok(())
}
```