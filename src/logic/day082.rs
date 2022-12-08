use crate::read_input;
use std::cmp::max;

pub struct Day082;
impl Day082 {

    
    pub fn run(&self) -> i32 {
        let filename = "input/day082.txt";
        let input = read_input(filename);
        
        let mut forrest: Vec<Vec<i32>> = Vec::new();
   
        for line in input.lines() {
            let row: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            forrest.push(row);
        }
        
        let mut max_scenic_score = 0;

        for y in 0..forrest.len() {
            for x in 0..forrest[y].len() {
                let scenic_score = calculate_scenic_score(&forrest, x, y); 
                max_scenic_score = max(max_scenic_score, scenic_score);
                
            }
        }
        
        // Print the result
        println!("Day 08 - Part 2: Result is {}", max_scenic_score);

        return max_scenic_score
    }
}

fn calculate_scenic_score(forrest: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let treesize = forrest[y][x];
    
    // Check score on y-axis
    let mut xleft = 0;
    let mut xright = 0;
    if x > 0 {
        for i in (0..x).rev() {
            xleft += 1;
            if forrest[y][i] >= treesize {
                break;
            }
        }
    }
    
    if x <= forrest[y].len() {
        for i in x+1..forrest[x].len() {
            xright += 1;
            if forrest[y][i] >= treesize {
                break;
            }
        }
    }
    
    // Check score on x-axis
    let mut yup = 0;
    let mut ydown = 0;
    if y > 0 {
        for i in (0..y).rev() {
            yup += 1;
            if forrest[i][x] >= treesize {
                break;
            }
        }
    }

    if y < forrest[x].len() {
        for i in y + 1..forrest[x].len() {
            ydown += 1;
            if forrest[i][x] >= treesize {
                break;
            }
        }
    }

    let scenic_score = xleft * xright * yup * ydown;
    return scenic_score
}
