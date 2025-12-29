<img src="logo_wide.png" alt="SerdeIO Logo" width="600">

[![Crates.io](https://img.shields.io/crates/v/serdeio)](https://crates.io/crates/serdeio)
[![Documentation](https://docs.rs/serdeio/badge.svg)](https://docs.rs/serdeio)
[![License](https://img.shields.io/crates/l/serdeio)](https://github.com/lucidfrontier45/serdeio/blob/main/LICENSE)

A lightweight Rust library for seamless serialization/deserialization of Serde-compatible structs across multiple data formats.

# Install

```sh
cargo add serdeio
```

SerdeIO supports JSON and JSON Lines formats by default. Additional formats can be enabled with feature flags:

```sh
# For CSV support
cargo add serdeio --features csv

# For YAML support
cargo add serdeio --features yaml

# For both CSV and YAML
cargo add serdeio --features csv,yaml
```

# Supported Formats

| Format     | Extensions       | Single Record | Multiple Records | Feature Flag |
| ---------- | ---------------- | ------------- | ---------------- | ------------ |
| JSON       | `.json`          | ✓             | ✓                | (default)    |
| JSON Lines | `.jsonl`, `.jsl` | ✗             | ✓                | (default)    |
| CSV        | `.csv`           | ✗             | ✓                | `csv`        |
| YAML       | `.yaml`, `.yml`  | ✓             | ✓                | `yaml`       |

# Features

- **Lightweight**: Minimal dependencies with feature-gated optional formats
- **Auto-detection**: File format automatically determined from extensions (case-insensitive)
- **Iterator support**: Efficient streaming writes without collecting into vectors
- **Serde-compatible**: Works with any struct that implements Serde traits
- **Flexible**: Supports both single records and collections across formats

# Performance

- Uses `BufReader`/`BufWriter` internally for optimal I/O performance
- Iterator-based writing enables memory-efficient streaming without allocations
- Format detection and parsing optimized for common use cases

# API Overview

SerdeIO provides 8 main functions for reading and writing data:

**Reader-based functions:**
- `read_record_from_reader<T>(reader, format)` - Read a single record from any `Read` implementation
- `read_records_from_reader<T>(reader, format)` - Read multiple records as `Vec<T>` from any `Read`
- `write_record_to_writer<T>(writer, format, record)` - Write a single record to any `Write`
- `write_records_to_writer<T>(writer, format, records)` - Write multiple records using an iterator

**File-based functions:**
- `read_record_from_file<T>(path)` - Read a single record, auto-detecting format from file extension
- `read_records_from_file<T>(path)` - Read multiple records, auto-detecting format from file extension
- `write_record_to_file<T>(path, record)` - Write a single record, auto-detecting format from file extension
- `write_records_to_file<T>(path, records)` - Write multiple records, auto-detecting format from file extension

Note: Some formats like CSV and JSON Lines only support multiple records (`Vec<T>`).

# Examples

## Reading and Writing Multiple Records (Common Use Case)

This example reads a JSON file containing multiple user records and converts it to JSON Lines format:

```rust
use anyhow::{Context, Result as AnyResult};
use serde::{Deserialize, Serialize};
use serdeio::{read_record_from_file, write_records_to_writer, DataFormat};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
    items: Vec<String>,
}

pub fn main() -> AnyResult<()> {
    // Get input file path from command line arguments
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];

    // Read JSON file to memory (format auto-detected from .json extension)
    let users: Vec<User> = read_record_from_file(input_file_path)
        .context("Failed to read records from file")?;

    // Write to stdout in JSON Lines format
    let writer = std::io::stdout();
    write_records_to_writer(writer, DataFormat::JsonLines, &users)?;

    Ok(())
}
```

# API Reference

For complete API documentation, visit [docs.rs/serdeio](https://docs.rs/serdeio).

Key types:
- `DataFormat` - Enum for specifying data formats
- `Error` - Comprehensive error type with format-specific variants

# Contributing

Contributions are welcome! Please:
- Run tests with `cargo test`
- Format code with `cargo fmt`
- Check with `cargo clippy`
- Follow the existing code style and patterns

# License

SerdeIO is licensed under the MIT License. See [LICENSE](LICENSE) for details.