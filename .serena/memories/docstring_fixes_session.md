# Docstring Fixes Session

## Task
Updated docstrings across the serdeio codebase to match the actual API function signatures.

## Changes Made

### src/write.rs
- `write_record_to_writer`: Fixed example argument order from `(writer, DataFormat, record)` to `(writer, record, DataFormat)`
- `write_records_to_writer`: Fixed example argument order from `(writer, DataFormat, records)` to `(writer, records, DataFormat)`
- `write_record_to_file`: Added missing `DataFormat::Auto` argument and `DataFormat` import to docstring examples
- `write_records_to_file`: Added missing `DataFormat::Auto` argument and `DataFormat` import to docstring examples

### src/read.rs
- `read_record_from_file`: Added missing `DataFormat::Auto` argument and `DataFormat` import to docstring examples
- `read_records_from_file`: Added missing `DataFormat::Auto` argument and `DataFormat` import to docstring examples

### README.md
- Fixed `read_records_from_file` call to include `DataFormat::Auto` argument
- Fixed `write_records_to_writer` argument order to match API: `(writer, records, DataFormat)`

## Verification
- All 10 doc tests pass: `cargo test --doc`
- Code passes linting: `cargo clippy -- -D warnings`
- Code formatted: `cargo fmt`

## Key Insight
The file-based functions (`*_from_file` and `*_to_file`) take 3 arguments: `(path, data, data_format?)` where `data_format` defaults to `Auto` if not provided. The writer functions (`*_to_writer` and `*_from_reader`) take the data format as the last argument, not the middle.