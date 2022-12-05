use crate::read_input;

pub struct Day042;
impl Day042 {

    pub fn run(&self) -> i32 {
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
        return result
    }
}