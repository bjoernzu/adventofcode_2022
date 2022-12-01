use std::fs::File;
use std::io::{BufReader};
use std::io::Read;

pub fn read_input(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    match reader.read_to_string(&mut input) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
    
    return input;
}