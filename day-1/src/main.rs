use std::env::current_dir;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn main() -> Result<(), Error>{
    let current_directory = current_dir()?;
    let input_path = format!("{}{}", current_directory.to_str().unwrap(), "/day-1/input.txt");
    let input = File::open(input_path)?;
    let buf_reader = BufReader::new(input);
    let lines = buf_reader.lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    for i in 0..lines.len() {
        let number1 = lines[i].parse::<u32>().unwrap();
        for j in i..lines.len(){
            let number2 = lines[j].parse::<u32>().unwrap();
            for z in j..lines.len(){
                let number3 = lines[z].parse::<u32>().unwrap();
                let sum = number1 + number2 + number3;
                if sum == 2020 {
                    let product = number1 * number2 * number3;
                    println!("{}", product);
                    return Ok(());
                }
            }
        }
    }
    return Ok(());
}
