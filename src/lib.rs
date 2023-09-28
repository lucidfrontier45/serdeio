pub mod common;
pub mod read;
pub mod write;

pub(crate) mod backend;

pub use common::{FileFormat, Result};
pub use read::{
    read_record_from_file, read_record_from_reader, read_records_from_file,
    read_records_from_reader,
};
pub use write::{
    write_record_to_file, write_record_to_writer, write_records_to_file, write_records_to_writer,
};
