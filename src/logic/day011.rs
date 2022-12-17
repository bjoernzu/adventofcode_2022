
use crate::read_input;
use crate::logic::Puzzle;
pub struct Day011;
impl Puzzle for Day011 {

    fn run(&self)  {
        
    let filename = "input/day011.txt";
    let input = read_input(filename);
    
    let mut max_calories = 0;
    let mut max_elf = 0;
    let mut elves: Vec<i32> = Vec::new();
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut current_elv_num = 1;
    let mut current_elv_calories = 0;
    for (_index, line) in input.lines().enumerate() {
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
    println!("Day 01 - Part 1: Elf {} carries max calories {}", max_elf, max_calories);
    }
}