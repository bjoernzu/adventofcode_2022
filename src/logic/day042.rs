use crate::read_input;
use crate::logic::Puzzle;
pub struct Day042;
impl Puzzle for Day042 {

    fn run(&self)  {
        let filename = "input/day042.txt";
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

            // Figure out if both overlap
            if (left[0] >= right[0] && left[0] <= right[1]) |
                (left[1] >= right[0] && left[1] <= right[1]) |
                (right[0] >= left[0] && right[0] <= left[1]) |
                (right[1] >= left[0] && right[1] <= left[1]) {
                result = result + 1;
            }
        }

        // Print the result
        println!("Day 04 - Part 1: Result is {} ", result);
        
    }
}