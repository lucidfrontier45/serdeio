# SerdeIO - Agent Guide

This guide helps agentic coding agents work effectively with the SerdeIO Rust library.

## Project Overview

SerdeIO is a tiny IO utility library for Rust that provides serialization/deserialization of Serde-compatible structs across multiple data formats (JSON, JSON Lines, CSV, YAML).

## Build/Test Commands

### Essential Commands
```bash
# Build and check the project
cargo check

# Run all tests
cargo test

# Run a specific test
cargo test <test_name> --lib
# Examples:
cargo test test_data_format --lib
cargo test test_read --lib

# Format code
cargo fmt

# Check with Clippy (linting)
cargo clippy -- -D warnings

# Build examples
cargo build --examples

# Run example with arguments
cargo run --example json2jsonl -- <input_file>
```

### Testing Strategy
- Unit tests are embedded within modules using `#[cfg(test)] mod test { ... }`
- Tests use `std::io::Cursor` for in-memory testing of read/write operations
- Test data is typically defined as string literals with `r#"...""#` syntax
- Tests verify both successful operations and error conditions

## Code Style Guidelines

### Import Organization
- Group imports: std libraries first, then external crates, then internal modules
- Use `use` statements at the top of files, organized by scope
- Prefer specific imports over glob imports
- Example import order:
```rust
use std::{fs::File, io::{Read, Write}, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::{backend, types::DataFormat, Error};
```

### Naming Conventions
- **Functions**: snake_case (e.g., `read_record_from_file`)
- **Types & Enums**: PascalCase (e.g., `DataFormat`)
- **Constants**: SCREAMING_SNAKE_CASE (rare in this codebase)
- **Variables**: snake_case (e.g., `data_format`, `reader`)
- **Modules**: snake_case (e.g., `jsonlines`, `backend`)

### Error Handling
- Use dedicated `Error` enum with `thiserror` for structured error types
- Error variants include `DataFormat`, `UnsupportedFormat`, `Io`, `Json`, `Csv`, `Yaml`
- Use `?` operator for error propagation through `#[from]` derives
- Specific error messages defined in error variants

### Type System
- Use generic type parameters with trait bounds: `T: DeserializeOwned`, `T: Serialize`
- Prefer `impl Trait` over concrete types in function parameters for flexibility
- Use lifetime parameters where needed: `'a` for iterator bounds
- Result types: `Result<T, crate::Error>` for operations that can fail

### Module Structure
- `pub(crate)` for internal modules that are not part of public API
- Public API re-exports in `lib.rs`
- Backend modules feature-gated: `#[cfg(feature = "csv")]`
- Consistent structure across backend modules (read/write functions + tests)

### Code Organization Patterns
- Backend modules follow a consistent pattern:
  - `read<T: DeserializeOwned>(reader) -> Result<T>` for single records
  - `read<T: DeserializeOwned>(reader) -> Result<Vec<T>>` for multiple records
  - `write<T: Serialize>(writer, record) -> Result<()>` for single records
  - `write<'a, T: Serialize + 'a>(writer, records) -> Result<()>` for multiple records

### Formatting and Style
- Use `rustfmt` for code formatting (configured in .vscode/settings.json)
- Line length follows rustfmt defaults
- Use 4-space indentation (rustfmt standard)
- Prefer `to_owned()` or `to_string()` over `clone()` for string conversions

### Feature Flags
- Optional backends are feature-gated: `#[cfg(feature = "csv")]`, `#[cfg(feature = "yaml")]`
- Always test both with and without features enabled
- Conditional compilation for format-specific code paths

### Documentation
- Use `///` for public API documentation
- Include examples in documentation where helpful
- Use `#[doc = include_str!("../README.md")]` for crate-level documentation

### Testing Patterns
- Test functions named descriptively: `test_read`, `test_write`, `test_data_format`
- Use `Cursor<Vec<u8>>` for testing write operations
- Use `Cursor<&str>` for testing read operations
- Test data structures include necessary derives: `Debug, Deserialize, Serialize, PartialEq, Eq`
- Assertions use `assert_eq!()` for comparing expected vs actual results

### Performance Considerations
- Use `BufReader` and `BufWriter` for I/O operations
- Avoid unnecessary allocations in hot paths
- Prefer iterator-based solutions over manual loops where appropriate
- Use `IntoIterator` bounds for flexible input types in write functions

## Development Workflow
1. Always run `cargo check` after changes
2. Run `cargo clippy` and fix all warnings
3. Run `cargo fmt` before committing
4. Run `cargo test` to verify functionality
5. Test with different feature combinations if applicable

## Common Pitfalls to Avoid
- Don't use unwrap() in library code - prefer proper error handling
- Don't forget to feature-gate optional dependencies
- Don't ignore clippy warnings - they often indicate real issues
- Don't forget to include test cases for error conditions