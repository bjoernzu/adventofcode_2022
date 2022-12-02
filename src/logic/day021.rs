use std::collections::HashMap;

use crate::read_input;

pub struct Day021;
impl Day021 {

    pub fn run(&self) -> i32 {
        let filename = "input/day021.txt";
        let input = read_input(filename);

        let mut opponent_score_mapping = HashMap::new();
        opponent_score_mapping.insert('A', 1);
        opponent_score_mapping.insert('B', 2);
        opponent_score_mapping.insert('C', 3);

        let mut my_score_mapping = HashMap::new();
        my_score_mapping.insert('X', 1);
        my_score_mapping.insert('Y', 2);
        my_score_mapping.insert('Z', 3);

        // Get the calories each elv is carrying
        let mut total_score :i32 = 0;
        for (_index, line) in input.lines().enumerate() {
            let mut round_score = 0;
            let my_symbol = line.chars().nth(2).unwrap();
            let opponent_symbol = line.chars().next().unwrap();
            let my = my_score_mapping.get(&my_symbol).unwrap();
            let opponent = opponent_score_mapping.get(&opponent_symbol).unwrap();
            // Own score
            total_score = total_score + my;
            round_score += my;
            // Game score
            if *my == *opponent + 1 || (*my == 1 && *opponent == 3) {
                total_score = total_score + 6;
                round_score += 6;
            }
            else if my == opponent {
                total_score = total_score + 3;
                round_score += 3;
            }
            println!("Opponent: {} {}, My: {} {}, Round Score {}, total score {}", opponent_symbol, opponent, my_symbol, my, round_score, total_score)
        }

        // Return the value for top 3 elves
        println!("Day 02 - Part 1: Total score is {} ", total_score);
        return total_score
    }
}