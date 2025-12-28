# Essential Development Commands

## Core Development Commands
```bash
# Build and check compilation
cargo check

# Run all tests
cargo test

# Run specific test
cargo test <test_name> --lib
# Examples:
cargo test test_data_format --lib
cargo test test_read --lib

# Format code according to rustfmt standards
cargo fmt

# Run linting and fix warnings (strict mode)
cargo clippy -- -D warnings

# Build examples
cargo build --examples

# Run example with arguments
cargo run --example json2jsonl -- <input_file>

# Build with all features (for comprehensive testing)
cargo check --all-features
cargo test --all-features
```

## Development Workflow Commands
```bash
# Standard development sequence
cargo check && cargo clippy -- -D warnings && cargo fmt && cargo test

# Test with specific features
cargo test --features csv
cargo test --features yaml
cargo test --all-features

# Documentation build
cargo doc --open

# Check documentation coverage
cargo doc --no-deps --document-private-items
```

## Git Commands (Linux environment)
```bash
# Check git status
git status

# Stage changes
git add <file>
git add .

# Commit changes
git commit -m "descriptive commit message"

# Pull latest changes
git pull

# Push changes
git push

# Check git log
git log --oneline -10
```

## File System Commands (Linux)
```bash
# List files
ls -la
ls src/backend/

# Find files by pattern
find . -name "*.rs"
find . -name "*.toml"

# Search in files
grep -r "pattern" src/
rg "pattern" src/  # if ripgrep is available

# Directory structure
tree -I target
find . -type d -name "*" | head -20
```