use std::io::{self, BufRead};
use std::fs::File;

pub(crate) fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    io::BufReader::new(file).lines().collect()
}

pub(crate) fn read_stdin() -> io::Result<Vec<String>> {
    io::stdin().lock().lines().collect()
}
