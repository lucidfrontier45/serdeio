use serde::{Deserialize, Serialize};

use recordio::{read_records_from_file, write_records_to_writer, FileFormat};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
    items: Vec<String>,
}

fn main() {
    // get input file path from argv
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];

    // read to memory
    let users: Vec<User> = read_records_from_file(input_file_path).unwrap();

    // write to stdout in YAML format
    let writer = std::io::stdout();
    write_records_to_writer(writer, FileFormat::Yaml, &users).unwrap();
}
