use crate::read_input;
use crate::logic::Puzzle;
pub struct Day032;
impl Puzzle for Day032 {

    // Priority of items
    // - Lowercase item types a through z have priorities 1 through 26.
    // - Uppercase item types A through Z have priorities 27 through 52.


    fn run(&self)  {
        let filename = "input/day032.txt";
        let input = read_input(filename);
    
        let mut result :i32 = 0;
        let mut elves = ["","",""];
        for (index, line) in input.lines().enumerate() {
            // Store elves rucksacks
            elves[index%3] = line;

            // When group of 3 elves is complete compare the content of their rucksacks
            if index%3 == 2 {
                for c in elves[0].chars() {
                    if elves[1].contains(c) && elves[2].contains(c) {
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
        }

        // Print the result
        println!("Day 03 - Part 2: Result is {} ", result);
    }
}