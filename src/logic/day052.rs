use crate::read_input;

pub struct Day052;
impl Day052 {

    pub fn run(&self) -> &str {
        let filename = "input/day052.txt";
        let input = read_input(filename);
    
        let mut stacks :Vec<Vec<String>> = Vec::new();
        let mut num_stacks :usize = 0;
        for line in input.lines() {
            if line.starts_with("move") {
                let elements: Vec<&str> = line.split_whitespace().collect();
                let num_ops: usize = elements[1].parse().unwrap();
                let from: usize = elements[3].parse().unwrap();
                let to: usize = elements[5].parse().unwrap();
                
                let split_off_index = stacks[from-1].len() - num_ops;
                let mut c = stacks[from-1].split_off(split_off_index);
                stacks[to-1].append(&mut c);
                
                
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
        print!("Day 05 - Part 2: Result is {}", result);

        for i in 0..=num_stacks - 1 {
            print!("{}", stacks[i].last().unwrap())
        }

        print!("\n");
        return result
    }
}
