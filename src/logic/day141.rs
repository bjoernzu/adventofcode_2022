use std::cmp::{min, max};

use crate::read_input;
use crate::logic::Puzzle;
pub struct Day141;
impl Puzzle for Day141 {
    fn run(&self) {
        let filename = "input/day141.txt";
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

        let x_corr = minx-1;

        // Build the empty cave
        for y in 0..=maxy-miny+1 {
            cave.push(Vec::new());
            for _x in 0..=maxx-minx+2 {
                cave[y].push('.');
            }
        }

        // _draw_cave(&cave);

        // Add the start point
        cave[sand_start.1][sand_start.0-x_corr] = 's';

        // Add the rock structures
        for rs in rock_structures {
            for x in min(rs.0.0, rs.1.0)-x_corr..=max(rs.0.0, rs.1.0)-x_corr {
                // println!("Adding rock structure {},{} to {},{}", rs.0.0, rs.0.1, rs.1.0, rs.1.1);

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
            let mut abyss = false;
            loop {
                if pos.1 >= maxy {
                    abyss = true;
                    break;
                }
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
                    cave[pos.1][pos.0] = 'o';
                    break;
                }
            }
            // _draw_cave(&cave);
            if abyss {
                break;
            }
            sand_units += 1;
        }

        // Print the result
        println!("Day 14 - Part 1: Result is {}", sand_units);
    }
}

fn _draw_cave(cave: &Vec<Vec<char>>) {
    for yvec in cave {
        println!("{}", yvec.iter().cloned().collect::<String>());
    }
}

