---
name: serdeio
description: Write Rust code that reads/writes Serde structs to/from JSON, JSON Lines, CSV, YAML, MessagePack, and TOML using the `serdeio` crate. Use this skill whenever the user mentions serdeio, asks to serialize or deserialize Rust structs to files, wants format conversion between JSON/JSONL/CSV/YAML/MessagePack/TOML in Rust, needs `DataFormat::Auto` file-extension detection, or wants streaming writes via `IntoIterator`. Trigger on phrases like "use serdeio", "read struct from JSON file", "write Vec<User> to CSV", "convert JSONL to CSV", "parse YAML config into a struct", "serialize to MessagePack", "TOML config struct", even when the user does not name serdeio explicitly.
---

# serdeio

`serdeio` is a thin wrapper that reads/writes any `Serialize`/`Deserialize` struct across six formats with one consistent API. Two flavors:

- **File APIs**: take a path, infer format from extension (or use an explicit `DataFormat`).
- **Reader/Writer APIs**: take any `Read`/`Write`, require an explicit `DataFormat` (`Auto` rejected).

Eight public functions, all return `Result<_, serdeio::Error>`.

## Cargo.toml

Format-specific features gate the non-default backends. JSON + JSON Lines are always available.

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serdeio = { version = "0.6", features = ["csv", "yaml", "messagepack", "toml"] }
# Drop features you do not need to keep the dep tree small.
```

Default features: `json`, `jsonlines` (CSV/YAML/MessagePack/TOML each require their flag).

## The eight functions

```rust
use serdeio::{DataFormat, Error};

// Reader-based — explicit format required
pub fn read_record_from_reader<T: DeserializeOwned>(reader: impl Read, format: DataFormat) -> Result<T, Error>;
pub fn read_records_from_reader<T: DeserializeOwned>(reader: impl Read, format: DataFormat) -> Result<Vec<T>, Error>;
pub fn write_record_to_writer<T: Serialize>(writer: impl Write, record: &T, format: DataFormat) -> Result<(), Error>;
pub fn write_records_to_writer<'a, T: Serialize + 'a>(writer: impl Write, records: impl IntoIterator<Item = &'a T>, format: DataFormat) -> Result<(), Error>;

// File-based — format inferred from extension, or pass DataFormat::Auto + override
pub fn read_record_from_file<T: DeserializeOwned>(path: impl AsRef<Path>, format: DataFormat) -> Result<T, Error>;
pub fn read_records_from_file<T: DeserializeOwned>(path: impl AsRef<Path>, format: DataFormat) -> Result<Vec<T>, Error>;
pub fn write_record_to_file<T: Serialize>(path: impl AsRef<Path>, record: &T, format: DataFormat) -> Result<(), Error>;
pub fn write_records_to_file<'a, T: Serialize + 'a, I: IntoIterator<Item = &'a T>>(path: impl AsRef<Path>, records: I, format: DataFormat) -> Result<(), Error>;
```

`DataFormat::Auto` is resolved by file extension (case-insensitive). Use it on file APIs, never on reader/writer APIs — those return `Error::AutoNotSupported`.

## DataFormat enum

```rust
pub enum DataFormat {
    Auto,
    Json,
    JsonLines,
    #[cfg(feature = "csv")]         Csv,
    #[cfg(feature = "yaml")]        Yaml,
    #[cfg(feature = "messagepack")] MessagePack,
    #[cfg(feature = "toml")]        Toml,
}
```

Recognized extensions (case-insensitive):
- `json` → `Json`
- `jsonl`, `jsl` → `JsonLines`
- `csv` → `Csv`
- `yaml`, `yml` → `Yaml`
- `msgpack`, `mpack`, `mpk`, `messagepack` → `MessagePack`
- `toml` → `Toml`

## Format capabilities

| Format      | Single record | Multiple records | Notes                                                   |
| ----------- | ------------- | ---------------- | ------------------------------------------------------- |
| JSON        | ✓             | ✓ (array)        | `read_records_*` expects a JSON array at the root.      |
| JSON Lines  | ✗             | ✓                | Streaming-friendly; blank lines tolerated on read.      |
| CSV         | ✗             | ✓                | Header row required. Struct field order = CSV column order. |
| YAML        | ✓             | ✓                |                                                         |
| MessagePack | ✓             | ✓                | Binary; Vec<u8> on the wire.                            |
| TOML        | ✓             | ✗                | Root must be a table; no top-level array of records.    |

## Patterns

### Read a single struct from a file

```rust
use serde::Deserialize;
use serdeio::{read_record_from_file, DataFormat};

#[derive(Deserialize)]
struct Config {
    name: String,
    version: u32,
}

let cfg: Config = read_record_from_file("config.toml", DataFormat::Auto)?;
```

Pass `DataFormat::Toml` to force the format and ignore the extension. Useful when the path has no extension or the wrong one.

### Read a Vec<T> from a file

```rust
use serde::Deserialize;
use serdeio::{read_records_from_file, DataFormat};

#[derive(Deserialize)]
struct User { id: u32, name: String }

let users: Vec<User> = read_records_from_file("users.jsonl", DataFormat::Auto)?;
```

Works for JSON (expects array), JSONL, CSV, YAML, MessagePack. TOML returns `Error::UnsupportedFormat` here — use `read_record_from_file` and put your records under a table field.

### Write a single struct to a file

```rust
use serde::Serialize;
use serdeio::{write_record_to_file, DataFormat};

#[derive(Serialize)]
struct Config {
    name: String,
    version: u32,
}

let cfg = Config { name: "myapp".into(), version: 1 };
write_record_to_file("config.toml", &cfg, DataFormat::Auto)?;
```

### Write a Vec<T> to a file

```rust
use serdeio::{write_records_to_file, DataFormat};

write_records_to_file("users.csv", &users, DataFormat::Auto)?;
```

`IntoIterator` signature means you can pass `&users`, `users.iter()`, or any other iterator over `&T` — the function never takes ownership of the records themselves. For JSON/YAML/MessagePack the iterator is collected internally (they need the full array); for JSONL/CSV it streams row-by-row.

### Read/write from any Read/Write

Reader/writer APIs are the right choice for stdin, network sockets, in-memory buffers, or anything that is not a `Path`. They require an explicit format — there is no extension to inspect.

```rust
use std::io::{stdin, stdout, BufWriter};
use serdeio::{read_record_from_reader, write_record_to_writer, DataFormat};

let user: User = read_record_from_reader(stdin().lock(), DataFormat::Json)?;

let mut out = BufWriter::new(stdout().lock());
write_record_to_writer(&mut out, &user, DataFormat::Json)?;
```

### Convert formats (read one, write another)

The most common use case: read records in one format, write them in another. Read with one format, write with another — the struct is the bridge.

```rust
use serde::{Deserialize, Serialize};
use serdeio::{read_records_from_file, write_records_to_writer, DataFormat};

#[derive(Deserialize, Serialize)]
struct Row { id: u32, name: String }

let rows: Vec<Row> = read_records_from_file("input.csv", DataFormat::Auto)?;
write_records_to_writer(std::io::stdout().lock(), &rows, DataFormat::JsonLines)?;
```

CSV in → JSON Lines out is a one-liner because both ends stream over `&[Row]`.

### Streaming write with an arbitrary iterator

`write_records_to_writer`/`_to_file` take any `IntoIterator<Item = &'a T>`, so you can stream from a generator without collecting:

```rust
for batch in db.scan_batches() {
    write_records_to_file("dump.jsonl", &batch, DataFormat::Auto)?;
}
```

## Pitfalls

- **`DataFormat::Auto` on reader/writer APIs → `Error::AutoNotSupported`.** Only file APIs can infer format from a path.
- **CSV column order follows struct field order.** If you reorder fields, the CSV layout changes. Rename with `#[serde(rename = "...")]` if you need stable column names.
- **JSON multi-record expects a JSON array at the root.** `read_records_*` with `Json` on a bare object returns a parse error. For streaming JSON, use `JsonLines` instead.
- **TOML has no multi-record mode.** There is no spec-legal way to write a TOML file whose root is an array of tables. For multiple records, encode them under a struct field: `struct Wrapper { items: Vec<Row> }`.
- **Feature flags are compile-time.** If you call `DataFormat::Yaml` without the `yaml` feature, the variant does not exist and your code will not compile. Mention the right flag in `Cargo.toml` up front.
- **The streaming-iterator signature is `&T`, not `T`.** `write_records_to_file("x.jsonl", &users, ...)` works; `write_records_to_file("x.jsonl", users, ...)` does not — `Vec<T>` is `IntoIterator<Item = T>`, not `&T`.
- **File APIs open with `BufReader`/`BufWriter`.** You do not need to wrap the file yourself; passing a `File` to a reader-API call still works (it is wrapped internally), but the file variants already buffer.
- **Case-insensitive extension match.** `data.JSON` resolves to `Json`. Keep the canonical lowercase form when you generate file names.

## Choosing between file and reader APIs

Use file APIs when you have a path and want extension-based detection or no manual buffering. Use reader/writer APIs when the source/sink is anything else (network, stdin, stdout, `Vec<u8>`, `Cursor`, etc.) or when you want the format choice to be explicit and local rather than implicit from a path.