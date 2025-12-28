# SerdeIO Project Overview

## Project Purpose
SerdeIO is a tiny IO utility library for Rust that provides serialization/deserialization of Serde-compatible structs across multiple data formats. It's a minimal, focused library that makes it easy to read and write structured data in various formats.

## Tech Stack
- **Language**: Rust (edition 2024)
- **Core Dependencies**: 
  - serde (with derive feature)
  - serde_json
  - anyhow (for error handling)
- **Optional Dependencies** (feature-gated):
  - serde_yaml (for YAML support)
  - csv (for CSV support)
- **Features**: csv, yaml (optional)
- **Target**: Library crate (no binary)

## Key Features
- JSON support (built-in)
- JSON Lines support (built-in)
- CSV support (optional feature)
- YAML support (optional feature)
- Automatic format detection from file extensions
- Consistent API across all formats
- Type-safe serialization/deserialization using Serde

## Public API Design
- `read_record_from_*` functions for single records
- `read_records_from_*` functions for multiple records (Vec<T>)
- `write_*` functions with corresponding write operations
- `DataFormat` enum for explicit format specification
- Automatic format detection from file paths

## Code Structure
- Modular design with separate backend modules for each format
- Consistent read/write API across all formats
- Feature-gated optional backends
- Comprehensive error handling with anyhow
- Extensive unit tests using in-memory Cursor testing