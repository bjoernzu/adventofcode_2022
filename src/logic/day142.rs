use std::cmp::{min, max};

use crate::read_input;
use crate::logic::Puzzle;
pub struct Day142;
impl Puzzle for Day142 {
    fn run(&self) {
        let filename = "input/day142.txt";
        let input = read_input(filename);

        let mut cave = Vec::new();
        let mut rock_structures = Vec::new();
        
        // Need to store the smallest and largest x and y
        let sand_start = (500, 0);
        let mut minx = sand_start.0;
        let mut maxx = sand_start.0;
        let mut miny = sand_start.1;
        let mut maxy = sand_start.1;

        for line in input.lines() {
            let coords: Vec<&str> = line.split(" -> ").collect();
            let mut start: (usize, usize) = (0, 0);
            let mut end;
            for coord in coords {
                let coord_vec: Vec<usize> = coord.split(",").map(|s| s.parse().unwrap()).collect();
                let coord_tup = (coord_vec[0], coord_vec[1]);
                
                minx = if coord_tup.0 < minx {coord_tup.0} else {minx};
                maxx = if coord_tup.0 > maxx {coord_tup.0} else {maxx};
                miny = if coord_tup.1 < miny {coord_tup.1} else {miny};
                maxy = if coord_tup.1 > maxy {coord_tup.1} else {maxy};
                
                if start == (0, 0) {
                    start = coord_tup;
                }
                else {
                    end = coord_tup;
                    rock_structures.push((start.clone(), end.clone()));
                    start = coord_tup;
                }
            }
        }

        let x_add_with = 1000;
        let x_corr = if minx >= x_add_with/2 {minx - x_add_with/2} else {x_add_with/2 - minx};

        // Build the empty cave
        for y in 0..=maxy-miny+2 {
            cave.push(Vec::new());
            for _x in 0..=maxx-minx+x_add_with {
                cave[y].push('.');
            }
        }

        // _draw_cave(&cave);

        // Add the start point
        cave[sand_start.1][sand_start.0-x_corr] = 's';

        // Add the floor 
        rock_structures.push(((x_corr, maxy+2), (maxx+x_add_with/2, maxy+2)));
        // Add the rock structures
        for rs in rock_structures {
            // println!("Adding rock structure {},{} to {},{}", rs.0.0, rs.0.1, rs.1.0, rs.1.1);
            for x in min(rs.0.0, rs.1.0)-x_corr..=max(rs.0.0, rs.1.0)-x_corr {

                for y in min(rs.0.1, rs.1.1)..=max(rs.0.1, rs.1.1) {
                    cave[y][x] = '#';
                }
            }
        }

        // _draw_cave(&cave);

        // Let the sand flow
        let mut sand_units = 0;
        loop {
            let mut pos = (sand_start.0 - x_corr, sand_start.1);
            let mut full = false;
            loop {
                if cave[pos.1+1][pos.0] == '.' {
                    pos.1 = pos.1 + 1;
                }
                else if cave[pos.1+1][pos.0-1] == '.' {
                    pos.1 = pos.1 + 1;
                    pos.0 = pos.0 - 1;
                }
                else if cave[pos.1+1][pos.0+1] == '.' {
                    pos.1 = pos.1 + 1;
                    pos.0 = pos.0 + 1;
                }
                else {
                    if pos == (sand_start.0 - x_corr, sand_start.1) {
                        full = true;
                        break;
                    }
                    cave[pos.1][pos.0] = 'o';
                    break;
                }
            }
            if sand_units % 250 == 0 {
                // _draw_cave(&cave);
            }
            sand_units += 1;
            if full {
                break;
            }
        }

        // Print the result
        println!("Day 14 - Part 2: Result is {}", sand_units);
    }
}

fn _draw_cave(cave: &Vec<Vec<char>>) {
    let mut y = 0;
    for yvec in cave {
        println!("{number:>3} {line}", number=y, line=yvec.iter().cloned().collect::<String>());
        y+=1;
    }
}

