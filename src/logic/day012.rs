use crate::read_input;
use crate::logic::Puzzle;
pub struct Day012;
impl Puzzle for Day012 {

    fn run(&self)  {
        let filename = "input/day012.txt";
        let input = read_input(filename);
        
        // Get the calories each elv is carrying
        let mut elves: Vec<i32> = Vec::new();
        let mut current_elv_calories = 0;
        for (_index, line) in input.lines().enumerate() {
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
        let top_3_elves: i32 = elves[..3].iter().sum();

        // Return the value for top 3 elves
        println!("Day 01 - Part 2: The top 3 elves are carrying {} callories", top_3_elves);
    }
}