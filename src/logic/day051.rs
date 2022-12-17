use crate::read_input;
use crate::logic::Puzzle;
pub struct Day051;
impl Puzzle for Day051 {

    fn run(&self) {
        let filename = "input/day051.txt";
        let input = read_input(filename);
    
        let mut stacks :Vec<Vec<String>> = Vec::new();
        let mut num_stacks :usize = 0;
        for line in input.lines() {
            if line.starts_with("move") {
                let elements: Vec<&str> = line.split_whitespace().collect();
                let num_ops: usize = elements[1].parse().unwrap();
                let from: usize = elements[3].parse().unwrap();
                let to: usize = elements[5].parse().unwrap();
                for _i in 1..=num_ops {
                    let c = stacks[from-1].pop().unwrap();
                    stacks[to-1].push(c);
                }
                
            }
            else if line.starts_with("[") {
                // Build start position
                if stacks.is_empty() {
                    num_stacks = (line.len() + 1) / 4;
                    for _i in 0..=num_stacks - 1 {
                        stacks.push(Vec::new());
                    }
                }

                let crates: Vec<String> = line.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect())
                    .collect();
                
                for i in 0..=num_stacks - 1 {
                    // Keep only the character
                    let c: String = crates[i].chars()
                        .filter(|c| c != &'[' && c != &']' && !c.is_whitespace())
                        .collect();
                    
                    // Add the crate to the stack
                    if !c.is_empty() {
                        stacks[i].push(c);
                    }

                }

            }
            else if line.starts_with(' ') {
                // Setup complete, reverse the stacks
                for i in 0..=num_stacks - 1 {
                    stacks[i].reverse();
                }
            }
        }
        
        let result = "";
        // Print the result
        print!("Day 05 - Part 1: Result is {}", result);

        for i in 0..=num_stacks - 1 {
            print!("{}", stacks[i].last().unwrap())
        }

        print!("\n");
        
    }
}
