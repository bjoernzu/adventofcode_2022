use crate::read_input;
use crate::logic::Puzzle;
pub struct Day041;
impl Puzzle for Day041 {

    fn run(&self)  {
        let filename = "input/day041.txt";
        let input = read_input(filename);
    
        let mut result :i32 = 0;
        
        for line in input.lines() {
            // Get the areas for each elv
            let elv_areas :Vec<&str> = line.split(",").collect();
            let left :Vec<i32> = elv_areas[0].split("-")
                .map(|s| s.parse().unwrap())
                .collect();
            let right :Vec<i32> = elv_areas[1].split("-")
                .map(|s| s.parse().unwrap())
                .collect();

            // Figure out if one is contained in the other
            if left[0] >= right[0] && left[1] <= right[1] {
                result = result + 1;
            }
            else if left[0] <= right[0] && left[1] >= right[1] {
                result = result + 1;
            }
        }

        // Print the result
        println!("Day 04 - Part 1: Result is {} ", result);
        
    }
}