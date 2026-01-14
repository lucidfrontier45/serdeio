# Suggested Commands for SerdeIO Development

## Basic Commands
See `AGENTS.md` for core commands:
- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`
- Check: `cargo check`

## Extended Build Options

### Build with specific features
```bash
cargo build --features csv,yaml
cargo build --features csv,yaml,messagepack,toml
cargo build --all-features
```

### Build release version
```bash
cargo build --release
```

## Extended Testing Options

### Run tests with all features
```bash
cargo test --all-features
```

### Run with output
```bash
cargo test -- --nocapture
```

### Run specific test
```bash
cargo test test_name
```

## Documentation

### Generate docs
```bash
cargo doc --all-features
```

### View docs locally
```bash
# Open in browser
start target/doc/serdeio/index.html
```

## Dependency Management

### Add dependency
```bash
cargo add serde
cargo add serde_yaml --features yaml
```

### Update dependencies
```bash
cargo update
```

### Show dependency tree
```bash
cargo tree
```

## Running Examples

### Run specific example
```bash
cargo run --example json2jsonl
cargo run --example json2jsonl --features csv,yaml
```

## Cleaning

```bash
cargo clean
```

## Windows Git Commands

### Stage changes
```bash
git add .
git add filename.rs
```

### Create commit
```bash
git commit -m "message"
```

### View history
```bash
git log --oneline
git log -5 --oneline
```

### Check status
```bash
git status
git diff
git diff --staged
```

### Branch operations
```bash
git branch
git checkout -b new-branch
git checkout main
```

### Sync with remote
```bash
git pull
git pull origin main
git push
git push -u origin new-branch
```

## Windows File Operations

### List files
```bash
dir
dir /s *.rs
```

### Find files containing text
```bash
findstr /s /c:"pattern" *.rs
```

### Create/remove directories
```bash
mkdir new_module
rmdir /s /q old_module
```

## Project Verification Commands

### Full verification (recommended before committing)
```bash
cargo check --all-features && cargo fmt && cargo clippy -- -D warnings && cargo test --all-features
```

### Check with all features
```bash
cargo check --all-features
cargo test --all-features
```

### Format check
```bash
cargo fmt -- --check
```

## Rust Toolchain

```bash
rustc --version
cargo --version
rustup update
```
