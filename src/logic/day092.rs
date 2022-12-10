use crate::read_input;
use std::collections::HashSet;
use std::cmp::min;

pub struct Day092;
impl Day092 {

    
    pub fn run(&self) -> usize {
        let filename = "input/day092.txt";
        let input = read_input(filename);
           
        let mut knots = Vec::new();
        for _i in 0..10 {
            knots.push((0,0));
        }
        let mut visited_places = HashSet::new();
        visited_places.insert((0,0));

        let mut step = 1;

        for line in input.lines() {
            let comp: Vec<&str> = line.split(" ").collect();
            let direction = comp[0];
            let steps: i32 = comp[1].parse().unwrap();
            for _i in 0..steps {
                let (mut hx, mut hy) = knots[0];

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

                knots[0] = (hx,hy);

                for i in 1..knots.len() {
                    let (prex, prey) = knots[i-1];
                    let (mut x, mut y) = knots[i];
                    // Calculate new tail position
                    let dy: i32 = prey - y;
                    let dx: i32 = prex - x;
                    if dy.abs() > 1 && dx.abs() > 1 {
                        x = min(x, prex) + 1;
                        y = min(y, prey) + 1;
                    }
                    else if dy.abs() > 0 && dx.abs() > 1 {
                        x = min(x, prex) + 1;
                        y = prey;
                    }
                    else if dy.abs() > 1 && dx.abs() > 0 {
                        x = prex;
                        y = min(y, prey) + 1;
                    }
                    else {
                        if dx.abs() > 1 {
                            x = min(x, prex) + 1;
                        }
                        if dy.abs() > 1 {
                            y = min(y, prey) + 1;
                        }
                    }
                    knots[i] = (x, y);
                }
                // Store new tail position                
                visited_places.insert(knots[9]);

                // step +=1;
                // let (tx, ty) = knots[9];
                //println!("Step {}: Found position: {}, {} (Line {}, Head: {}, {})", step, tx, ty, line, hx, hy);
            }
        }

        // Count number of different locations
        let result = visited_places.len();

        // Print the result
        println!("Day 09 - Part 2: Result is {}", result);

        return result
    }
}

