use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Day012;
impl Day012 {

    pub fn run(&self) -> i32 {
        let filename = "input/day012.txt";
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        
        // Get the calories each elv is carrying
        let mut elves: Vec<i32> = Vec::new();
        let mut current_elv_calories = 0;
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.is_empty() {
                elves.push(current_elv_calories);
                current_elv_calories = 0;
            }
            else {
                current_elv_calories += line.parse().unwrap_or(0);
            }
        }

        // Sort the elves by calories and get the sum for the top most 3
        elves.sort();
        elves.reverse();
        let top_3_elves = elves[..3].iter().sum();

        // Return the value for top 3 elves
        println!("The top 3 elves are carrying {} callories", top_3_elves);
        return top_3_elves
    }
}