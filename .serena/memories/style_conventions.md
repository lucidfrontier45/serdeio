# SerdeIO Code Style and Conventions

## Naming Conventions

### General Rules
- **snake_case** for variables, functions, modules
- **PascalCase** for types, traits, enum variants
- **SCREAMING_SNAKE_CASE** for constants

### Examples
```rust
// Good
let data_format = DataFormat::Json;
fn read_records_from_file<T>(path: impl AsRef<Path>) -> Result<Vec<T>, Error>

// Bad
let df = DataFormat::Json;
fn read<T>(p: &str) -> Vec<T>
```

## Documentation Patterns

All public API must have:
1. Summary sentence
2. `# Supported Formats` section (for format-specific functions)
3. `# Errors` section
4. `# Examples` section with runnable code

```rust
/// Reads multiple records from a file and deserializes them into a vector.
///
/// # Supported Formats
///
/// - JSON (.json)
/// - JSON Lines (.jsonl, .jsl)
/// - CSV (.csv) - requires `csv` feature
///
/// # Errors
///
/// Returns an error if the file cannot be opened or deserialization fails.
///
/// # Examples
///
/// ```rust,no_run
/// use serdeio::read_records_from_file;
/// let users: Vec<User> = read_records_from_file("users.json")?;
/// ```
pub fn read_records_from_file<T>(path: impl AsRef<Path>) -> Result<Vec<T>, Error>
```

## Error Handling

Use `thiserror` for Error types:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("Data format error: {0}")]
    DataFormat(#[from] crate::types::DataFormatError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[cfg(feature = "csv")]
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}
```

Key patterns:
- Derive `Error` and `Debug`
- Use `#[from]` for automatic conversion
- Feature-gate error variants

## Feature Flag Pattern

```toml
[features]
csv = ["dep:csv"]
yaml = ["dep:serde_yaml"]
```

```rust
#[cfg(feature = "csv")]
DataFormat::Csv => backend::csv::read(reader),

#[allow(unreachable_patterns)]
_ => Err(Error::UnsupportedFormat(data_format)),
```

## Backend Module Pattern

Each format module implements read/write:

```rust
// src/backend/json.rs
use std::io::{BufReader, BufWriter, Read, Write};
use serde::{Serialize, de::DeserializeOwned};
use crate::Error;

pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let reader = BufReader::new(reader);
    Ok(serde_json::from_reader(reader)?)
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let writer = BufWriter::new(writer);
    Ok(serde_json::to_writer(writer, record)?)
}
```

## Performance Patterns

### Iterator-based writing
```rust
pub fn write_records_to_writer<'a, T: Serialize + 'a>(
    writer: impl Write,
    data_format: DataFormat,
    records: impl IntoIterator<Item = &'a T>,
) -> Result<(), Error> {
    match data_format {
        DataFormat::JsonLines => backend::jsonlines::write(writer, records),
        _ => backend::json::write(writer, &records.into_iter().collect::<Vec<_>>()),
    }
}
```

## Import Organization
```rust
use std::{fmt::Display, path::Path};

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::{types::DataFormat, Error};

use super::{parent_module, another_module};
```

## What NOT to Do
- ❌ Never use `.unwrap()` without a safety comment
- ❌ Never commit without running tests
- ❌ Never add dependencies without asking first

## What to ALWAYS Do
- ✅ Document all public API functions
- ✅ Feature-gate optional format support
- ✅ Use `BufReader`/`BufWriter` for I/O
- ✅ Run all checks before committing
