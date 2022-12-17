use crate::read_input;
use crate::logic::Puzzle;use std::collections::HashSet;
use std::cmp::min;

pub struct Day091;
impl Puzzle for Day091 {

    
    fn run(&self)  {
        let filename = "input/day091.txt";
        let input = read_input(filename);
           
        let mut hy = 0;
        let mut hx = 0;
        let mut ty = 0;
        let mut tx = 0;

        let mut visited_places = HashSet::new();
        visited_places.insert((0,0));

        // let mut step = 1;

        for line in input.lines() {
            let comp: Vec<&str> = line.split(" ").collect();
            let direction = comp[0];
            let steps: i32 = comp[1].parse().unwrap();
            for _i in 0..steps {
                match direction {
                    "U" => {
                        hy += 1;
                    }
                    "D" => {
                        hy -= 1;
                    }
                    "L" => {
                        hx -= 1;
                    }
                    "R" => {
                        hx += 1;
                    }
                    _ => {}
                }

                // Calculate new tail position
                let dy: i32 = hy - ty;
                let dx: i32 = hx - tx;

                if dy.abs() > 0 && dx.abs() > 1 {
                    tx = min(tx, hx) + 1;
                    ty = hy;
                }
                else if dy.abs() > 1 && dx.abs() > 0 {
                    tx = hx;
                    ty = min(ty, hy) + 1;
                }
                else {
                    if dx.abs() > 1 {
                        tx = min(tx, hx) + 1;
                    }
                    if dy.abs() > 1 {
                        ty = min(ty, hy) + 1;
                    }
                }

                // Store new tail position                
                visited_places.insert((tx, ty));
                
                // step +=1;
                //println!("Step {}: Found position: {}, {} (Line {}, Head: {}, {})", step, tx, ty, line, hx, hy);
            }
        }

        // Count number of different locations
        let result = visited_places.len();

        // Print the result
        println!("Day 09 - Part 1: Result is {}", result);

        
    }
}

