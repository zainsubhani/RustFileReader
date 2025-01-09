use std::fs::File;
use std::io::ErrorKind;
use std::io::{BufRead, BufReader};
fn file_reader() {
    let file = File::open("Cargo.toml");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found: {}", error);
                }
                other_error => {
                    println!("Problem opening the file: {:?}", other_error);
                }
            }
            panic!("Problem opening the file: {:?}", error)
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
