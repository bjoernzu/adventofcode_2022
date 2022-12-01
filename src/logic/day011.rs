use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Day011;
impl Day011 {

    pub fn run(&self) -> i32 {
        
    let filename = "input/day011.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut max_calories = 0;
    let mut max_elf = 0;
    let mut elves: Vec<i32> = Vec::new();
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut current_elv_num = 1;
    let mut current_elv_calories = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.is_empty() {
            elves.push(current_elv_calories);
            if current_elv_calories > max_calories {
                max_calories = current_elv_calories;
                max_elf = current_elv_num;
            }
            current_elv_calories = 0;
            current_elv_num = current_elv_num + 1;
        }
        else {
            current_elv_calories += line.parse().unwrap_or(0);
        }
    }
    // Return the max
    println!("Elf {} carries max calories {}", max_elf, max_calories);
    return max_calories
    }
}