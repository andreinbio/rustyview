use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

/// # Get file
pub fn load_file(file_path: &str) -> String {

    let mut f = File::open(file_path).expect("Unable to open");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    contents
}