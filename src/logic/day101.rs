use crate::read_input;
use crate::logic::Puzzle;

pub struct Day101;
impl Puzzle for Day101 {

    
    fn run(&self)  {
        let filename = "input/day101.txt";
        let input = read_input(filename);

        let mut step: i32 = 1;
        let mut x: i32 = 1;
        let mut signal_strength = 0;

        for line in input.lines() {
            
            // Execute noop
            if line.starts_with("noop") {
                signal_strength += check_signal_strength(&step, &x);
                step += 1;
            }
            else if line.starts_with("addx") {
                signal_strength += check_signal_strength(&step, &x);
                step += 1;
                signal_strength += check_signal_strength(&step, &x);
                step += 1;
                let num = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                x += num;
            }
        }

        // Print the result
        println!("Day 10 - Part 1: Result is {}", signal_strength);
    }
}

fn check_signal_strength(step: &i32, x: &i32) -> i32 {
    let checkpoints = [20, 60, 100, 140, 180, 220];
    if checkpoints.contains(step) {
        //println!("Step {}, x {}, signal strength {}", step, x, x*step);
        return x * step;
    }
    return 0;
}