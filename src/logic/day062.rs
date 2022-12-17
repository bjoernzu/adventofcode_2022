use crate::read_input;
use crate::logic::Puzzle;use std::collections::HashSet;

pub struct Day062;
impl Puzzle for Day062 {

    fn run(&self)  {
        let filename = "input/day061.txt";
        let input = read_input(filename);
        
        let mut result :usize = 0;
        let marker_size = 14;
        for i in marker_size..=input.len() -1 {
            if is_unique(input.get(i-marker_size..i).unwrap()) {
                result = i;
                break;
            }
        }
        
        // Print the result
        println!("Day 06 - Part 2: Result is {}", result);

        
    }
}

fn is_unique(str: &str) -> bool {
    let r = str.chars().collect::<HashSet<char>>().len() == str.len();
    // println!("{} is unique: {}", str, r);
    return r;
}
