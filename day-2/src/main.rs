use std::env::current_dir;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error>{
    let current_directory = current_dir()?;
    let input_path = format!("{}{}", current_directory.to_str().unwrap(), "/day-1/input.txt");
    let input = File::open(input_path)?;
    let buf_reader = BufReader::new(input);
    let lines = buf_reader.lines();

    return Ok(());
}
