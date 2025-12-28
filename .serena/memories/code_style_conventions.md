# Code Style and Conventions

## Import Organization
- **Order**: std libraries → external crates → internal modules
- **Style**: Group related imports, prefer specific imports over glob imports
- **Example**:
```rust
use std::{fs::File, io::{Read, Write}, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::{backend, types::DataFormat, Error};
```

## Naming Conventions
- **Functions**: snake_case (e.g., `read_record_from_file`)
- **Types & Enums**: PascalCase (e.g., `DataFormat`)
- **Constants**: SCREAMING_SNAKE_CASE (rare in this codebase)
- **Variables**: snake_case (e.g., `data_format`, `reader`)
- **Modules**: snake_case (e.g., `jsonlines`, `backend`)

## Error Handling Patterns
- **Primary Error Type**: `Result<T, crate::Error>` using dedicated error enum
- **Error Derivation**: Use `thiserror` for structured error types
- **Error Variants**: Define specific variants for different error conditions
- **Error Propagation**: Use `#[from]` derives and `?` operator
- **Error Messages**: Define in error variants with `#[error(...)]` attributes
```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Json(#[from] serde_json::Error),
    // ... other variants
}
```

## Type System Guidelines
- **Generic Constraints**: Use trait bounds like `T: DeserializeOwned`, `T: Serialize`
- **Function Parameters**: Prefer `impl Trait` over concrete types for flexibility
- **Lifetime Parameters**: Use `'a` for iterator bounds when needed
- **Result Types**: Always use `Result<T, crate::Error>` for fallible operations

## Module Structure
- **Visibility**: Use `pub(crate)` for internal modules
- **Re-exports**: Public API re-exports in `lib.rs`
- **Feature Gates**: Optional backends use `#[cfg(feature = "csv")]`
- **Consistency**: All backend modules follow same structure (read/write functions + tests)

## Code Organization Patterns
- **Backend Modules**: Consistent API pattern:
  - Single record: `read<T: DeserializeOwned>(reader) -> Result<T>`
  - Multiple records: `read<T: DeserializeOwned>(reader) -> Result<Vec<T>>`
  - Write single: `write<T: Serialize>(writer, record) -> Result<()>`
  - Write multiple: `write<'a, T: Serialize + 'a>(writer, records) -> Result<()>`

## Formatting and Style
- **Tool**: rustfmt with configuration in .vscode/settings.json
- **Indentation**: 4 spaces (rustfmt standard)
- **Line Length**: Follow rustfmt defaults
- **String Conversions**: Prefer `to_owned()` or `to_string()` over `clone()`

## Feature Flag Usage
- **Optional Backends**: Feature-gate with `#[cfg(feature = "csv")]`, `#[cfg(feature = "yaml")]`
- **Testing**: Always test both with and without features enabled
- **Compilation**: Use conditional compilation for format-specific code paths

## Documentation Standards
- **Public API**: Use `///` for documentation comments
- **Examples**: Include usage examples where helpful
- **Crate Docs**: Use `#[doc = include_str!("../README.md")]` for crate-level documentation

## Testing Patterns
- **Test Naming**: Descriptive names like `test_read`, `test_write`, `test_data_format`
- **Test Data**: Use `Cursor<Vec<u8>>` for writes, `Cursor<&str>` for reads
- **Test Data Structures**: Include derives: `Debug, Deserialize, Serialize, PartialEq, Eq`
- **Assertions**: Use `assert_eq!()` for comparing expected vs actual
- **Test Organization**: Use `#[cfg(test)] mod test { ... }` within modules

## Performance Considerations
- **I/O Operations**: Use `BufReader` and `BufWriter`
- **Memory**: Avoid unnecessary allocations in hot paths
- **Iterators**: Prefer iterator-based solutions over manual loops
- **Flexibility**: Use `IntoIterator` bounds for flexible input types in write functions