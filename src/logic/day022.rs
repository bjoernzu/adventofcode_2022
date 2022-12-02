use std::collections::HashMap;

use crate::read_input;

pub struct Day022;
impl Day022 {

    pub fn run(&self) -> i32 {
        let filename = "input/day021.txt";
        let input = read_input(filename);

        // X means you need to lose, 
        // Y means you need to end the round in a draw, 
        // and Z means you need to win

        let mut option_mapping = HashMap::new();
        option_mapping.insert("A X", 0 + 3);
        option_mapping.insert("A Y", 3 + 1);
        option_mapping.insert("A Z", 6 + 2);
        option_mapping.insert("B X", 0 + 1);
        option_mapping.insert("B Y", 3 + 2);
        option_mapping.insert("B Z", 6 + 3);
        option_mapping.insert("C X", 0 + 2);
        option_mapping.insert("C Y", 3 + 3);
        option_mapping.insert("C Z", 6 + 1);
        
        // Get the calories each elv is carrying
        let mut total_score :i32 = 0;
        for (_index, line) in input.lines().enumerate() {
            let mut round_score = option_mapping.get(line).unwrap();
            total_score = total_score + round_score
            //println!("Opponent: {} {}, My: {} {}, Round Score {}, total score {}", opponent_symbol, opponent, my_symbol, my, round_score, total_score)
        }

        // Return the value for top 3 elves
        println!("Day 02 - Part 2: Total score is {} ", total_score);
        return total_score
    }
}