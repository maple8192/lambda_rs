use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    read_to_string(path)
}
