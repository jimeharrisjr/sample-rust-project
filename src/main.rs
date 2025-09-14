use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::panic::panic_any;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[0]);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                     println!("Permission denied when opening the file: {}", error);
                     return;
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}