# Task Completion Checklist for SerdeIO

## Pre-Commit Checklist

Run basic checks (see `AGENTS.md`):
```bash
cargo check
cargo fmt
cargo clippy -- -D warnings
cargo test
```

Then run extended verification:
```bash
cargo check --all-features
cargo test --all-features
```

## Code Quality for SerdeIO

### Memory Safety
- Verify ownership in format implementations
- Check reader/writer lifetimes
- Validate trait bounds on generic types

### Performance
- Use `BufReader`/`BufWriter` (already in backend modules)
- Minimize allocations in hot paths
- Use iterators for streaming writes

## Testing Requirements

### Unit Tests
- Add tests in `#[cfg(test)]` blocks within modules
- Test format detection and parsing
- Test error conditions for each format

### Integration Tests
- Place in `src/` directory tree
- Test file I/O with real format files
- Test across all supported formats

### Test Coverage
- Test both single record and multiple records APIs
- Test all format combinations
- Test error handling for each format

## Feature Gate Checklist

When adding new format support:

1. Add feature to `Cargo.toml`:
   ```toml
   [features]
   newformat = ["dep:external_crate"]
   ```

2. Add feature-gated error variant if needed

3. Add feature-gated `DataFormat` variant

4. Add feature-gated pattern matching in read.rs and write.rs

5. Add backend module in `src/backend/newformat.rs`

6. Add feature-gated tests

7. Update documentation

## Module Organization

### Adding New Format Backend
1. Create `src/backend/{format}.rs`
2. Implement `read` and `write` functions
3. Use `BufReader`/`BufWriter`
4. Return `Result<T, Error>`
5. Export in read.rs/write.rs with feature gates

### Backend Pattern
```rust
pub fn read<T: DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let reader = BufReader::new(reader);
    // format-specific parsing
}

pub fn write<T: Serialize>(writer: impl Write, record: &T) -> Result<(), Error> {
    let writer = BufWriter::new(writer);
    // format-specific serialization
}
```

## Common Issues to Check

### Format Detection
- Verify file extension matching is case-insensitive
- Check all supported extensions for each format

### Error Handling
- Ensure all format-specific errors convert to `Error` enum
- Feature-gate error variants correctly

### Type Consistency
- Match generic bounds across read/write APIs
- Verify `DeserializeOwned` and `Serialize` trait bounds

### Feature Gating
- Ensure disabled features don't break enabled code
- Test both with and without optional features

## Post-Implementation Verification

1. **Build**: `cargo build --all-features`
2. **Format**: `cargo fmt`
3. **Lint**: `cargo clippy -- -D warnings`
4. **Test**: `cargo test --all-features`
5. **Docs**: `cargo doc --all-features`

## Git Workflow

### Commit message format
```
<type>: <description>

Types: feat, fix, docs, style, refactor, test, chore
```

### Example commits
```
feat: Add TOML format support for single records
fix: Handle empty CSV files gracefully
docs: Update README with new format examples
```

## When to Ask for Help
- Adding dependencies that impact compile times
- Breaking changes to public API
- Complex lifetime or generic bounds
- Unclear format requirements
