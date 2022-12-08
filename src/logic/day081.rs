use crate::read_input;
use std::cmp::max;

pub struct Day081;
impl Day081 {

    
    pub fn run(&self) -> i32 {
        let filename = "input/day081.txt";
        let input = read_input(filename);
        
        let mut forrest: Vec<Vec<i32>> = Vec::new();
   
        for line in input.lines() {
            let row: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            forrest.push(row);
        }
        
        let mut visible_trees = 0;

        for y in 0..forrest.len() {
            for x in 0..forrest[y].len() {
                let treevis = is_visible(&forrest, x, y); 
                // print!("{}", treevis);
                visible_trees += treevis
            }
            // print!{"\n"};
        }
        
        // Print the result
        println!("Day 08 - Part 1: Result is {}", visible_trees);

        return visible_trees
    }
}

fn is_visible(forrest: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let treesize = forrest[y][x];
    if x == 0 || x ==forrest[y].len() - 1 || y == 0 || y == forrest.len() - 1 {
        // println!("Tree x: {}, y: {} is visible: {}", x,y,1);
        return 1;
    }
    // Check visibility on y-axis
    let mut xleft = 1;
    let mut xright = 1;
    for i in 0..=forrest.len() - 1 {
        if i < x && forrest[y][i] >= treesize {
            xleft = 0;
        }
        if i > x && forrest[y][i] >= treesize {
            xright = 0;
        }
    }
    let xvis = max(xleft, xright);
    
    // Check visibility on x-axis
    let mut yup = 1;
    let mut ydown = 1;
    for i in 0..=forrest[x].len() - 1 {
        if i < y && forrest[i][x] >= treesize {
            yup = 0;
        }
        if i > y && forrest[i][x] >= treesize {
            ydown = 0;
        }
    }
    let yvis = max(yup, ydown);
    
    let visible = max(yvis, xvis);
    return visible
}
