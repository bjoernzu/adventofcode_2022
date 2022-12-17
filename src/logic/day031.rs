use crate::read_input;
use crate::logic::Puzzle;
pub struct Day031;
impl Puzzle for Day031 {

    
    fn run(&self)  {
        let filename = "input/day031.txt";
        let input = read_input(filename);
        
        let mut result :i32 = 0;
        for (_index, line) in input.lines().enumerate() {
            // Split line in half
            let left = &line[0..line.len()/2];
            let right = &line[line.len()/2..];
            
            // Loop through left string to find character that is also in right string
            for c in left.chars() {
                if right.contains(c) {
                    let cnum = c as i32;
                    // Priority of items calculated based on their Ascii values + correction
                    // - Lowercase item types a through z have priorities 1 through 26.
                    // - Uppercase item types A through Z have priorities 27 through 52.
                    if cnum <= 90 {
                        result = result + cnum - 38;
                    }
                    else if cnum >=97 && cnum <= 122 {
                        result = result + cnum - 96;
                    }
                    break;
                }
            }
        }

        println!("Day 02 - Part 1: Result is {} ", result);
    }
}