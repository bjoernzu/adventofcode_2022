use crate::read_input;
use std::collections::HashSet;

pub struct Day061;
impl Day061 {

    pub fn run(&self) -> usize {
        let filename = "input/day061.txt";
        let input = read_input(filename);
        
        let mut result :usize = 0;
   
        for i in 4..=input.len() -1 {
            if is_unique(input.get(i-4..i).unwrap()) {
                result = i;
                break;
            }
        }
        
        // Print the result
        println!("Day 06 - Part 1: Result is {}", result);

        return result
    }
}

fn is_unique(str: &str) -> bool {
    let r = str.chars().collect::<HashSet<char>>().len() == str.len();
    // println!("{} is unique: {}", str, r);
    return r;
}
