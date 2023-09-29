use anyhow::{anyhow, Context, Result as AnyResult};
use serde::{Deserialize, Serialize};
use serdeio::{read_record_from_file, write_records_to_writer, DataFormat};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
    items: Vec<String>,
}

pub fn main() -> AnyResult<()> {
    // get input file path from argv
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];

    // read json to memory
    let users: Vec<User> = read_record_from_file(input_file_path)
        .map_err(|e| anyhow! {e})
        .context("Failed to read records from file")?;

    // write to stdout in json lines format
    let writer = std::io::stdout();
    write_records_to_writer(writer, DataFormat::JsonLines, &users).unwrap();

    Ok(())
}
