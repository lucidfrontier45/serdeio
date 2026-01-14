# SerdeIO Project Overview

## Purpose
SerdeIO is a lightweight Rust library for seamless serialization/deserialization of Serde-compatible structs across multiple formats with automatic format detection from file extensions.

## Key Features
- Auto-detection of file format from extensions (case-insensitive)
- Iterator support for efficient streaming writes
- Uses `BufReader`/`BufWriter` for optimal I/O performance
- Minimal dependencies with feature-gated optional formats

## Supported Formats
| Format      | Extensions              | Single Record | Multiple Records | Feature Flag |
|-------------|------------------------|---------------|------------------|--------------|
| JSON        | .json                  | ✓             | ✓                | (default)    |
| JSON Lines  | .jsonl, .jsl           | ✗             | ✓                | (default)    |
| CSV         | .csv                   | ✗             | ✓                | `csv`        |
| YAML        | .yaml, .yml            | ✓             | ✓                | `yaml`       |
| MessagePack | .msgpack, .mpack, .mpk | ✓             | ✓                | `messagepack`|
| TOML        | .toml                  | ✓             | ✗                | `toml`       |

## Main API Functions

**Reader-based:**
- `read_record_from_reader<T>(reader, format)` - Read single record from any `Read`
- `read_records_from_reader<T>(reader, format)` - Read multiple records as `Vec<T>`
- `write_record_to_writer<T>(writer, format, record)` - Write single record
- `write_records_to_writer<T>(writer, format, records)` - Write multiple records via iterator

**File-based (auto-detect format from extension):**
- `read_record_from_file<T>(path)` - Read single record
- `read_records_from_file<T>(path)` - Read multiple records
- `write_record_to_file<T>(path, record)` - Write single record
- `write_records_to_file<T, I: IntoIterator<Item=&T>>(path, records)` - Write multiple records

## Project Structure
```
serdeio/
├── src/
│   ├── lib.rs              # Main entry point
│   ├── read.rs             # read functions
│   ├── write.rs            # write functions
│   ├── types.rs            # DataFormat enum
│   ├── error.rs            # Error types
│   └── backend/            # Format implementations
│       ├── json.rs
│       ├── jsonlines.rs
│       ├── csv.rs
│       ├── yaml.rs
│       ├── messagepack.rs
│       └── toml.rs
├── examples/
├── Cargo.toml
└── README.md
```
