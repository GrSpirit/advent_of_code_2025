mod input;
mod task;

use input::{read_file, read_stdin};
use std::io;
use std::env;
use task::*;

enum Mode {
    File(String),
    Stdin
}

fn main() -> io::Result<()> {
    let mode = env::args().nth(1).map(|arg| Mode::File(arg)).unwrap_or(Mode::Stdin);
    let data = match mode {
        Mode::File(file_path) => read_file(&file_path)?,
        Mode::Stdin => read_stdin()?
    };

    match task1(&data) {
        Ok(result) => println!("result1 {}", result),
        Err(error) => println!("error {}", error)
    }

    match task2(&data) {
        Ok(result) => println!("result2 {}", result),
        Err(error) => println!("error {}", error)
    }

    Ok(())
}
