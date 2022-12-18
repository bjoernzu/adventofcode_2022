use std::collections::HashMap;

use crate::read_input;
use crate::logic::Puzzle;

type Pixel = (i8,i8,i8);

pub struct Day181;
impl Puzzle for Day181 {
    fn run(&self) {
        let filename = "input/day181.txt";
        let input = read_input(filename);

        let mut pixel_groups: HashMap<Pixel, u32> = HashMap::new();
        let mut groups: HashMap<u32, u32> = HashMap::new();

        let mut max_group = 1;
        
        let mut lines: Vec<&str> = input.lines().collect();
        lines.sort();
        let mut double_counted_pixels: u32 = 0;

        for line in lines {
            if !line.is_empty() {
                let dots: Vec<i8> = line.split(",").map(|c| c.parse::<i8>().unwrap()).collect();
                let pixel = (dots[0], dots[1], dots[2]);
                let neighbour_group;
                let mut neighbour_groups = HashMap::new();
                for p2 in pixel_groups.iter() {
                    if touches(&pixel, p2.0) {
                        // println!("P1 {},{},{} touches P2 {},{},{}", pixel.0, pixel.1, pixel.2, p2.0.0, p2.0.1, p2.0.2);
                        if neighbour_groups.contains_key(p2.1) {
                            *neighbour_groups.entry(p2.1 + 0).or_insert(1) += 1;
                            *groups.entry(p2.1 + 0).or_insert(6) -= 2;
                        }
                        else {
                            neighbour_groups.insert(p2.1 + 0, 1);
                            *groups.entry(p2.1 + 0).or_insert(6) += 4;
                        }
                    }
                    else {
                        // println!("P1 {},{},{} does not touch P2 {},{},{}", pixel.0, pixel.1, pixel.2, p2.0.0, p2.0.1, p2.0.2);
                    }
                }


                if neighbour_groups.len() == 0 {
                    neighbour_group = max_group;
                    groups.insert(neighbour_group, 6);
                    pixel_groups.insert(pixel, neighbour_group);
                    max_group += 1;
                }
                else if neighbour_groups.len() == 1 {
                    neighbour_group = neighbour_groups.iter().last().unwrap().1 + 0 as u32;
                    pixel_groups.insert(pixel, neighbour_group);
                }
                else {
                    // println!("Found {} neighbor groups for pixel {},{},{}", &neighbour_groups.len(), pixel.0, pixel.1, pixel.2);
                    neighbour_group = neighbour_groups.iter().last().unwrap().1 +0 ;
                    pixel_groups.insert(pixel, neighbour_group);
                    double_counted_pixels += neighbour_groups.len() as u32 - 1;
                }
            }
        }

        let mut result = 0;
        result += groups.iter().map(|g| g.1).sum::<u32>();
        result -= double_counted_pixels * 6;
        println!("Day 18 - Part 1: Result is {}", result);
    }
}

fn touches(p1: &Pixel, p2: &Pixel) -> bool {
    (p1.0 == p2.0               && p1.1 == p2.1             && (p1.2 - p2.2).abs() == 1) ||
    (p1.0 == p2.0               && (p1.1 - p2.1).abs() == 1 && p1.2 == p2.2 ) ||
    ((p1.0 - p2.0).abs() == 1   && p1.1 == p2.1             && p1.2 == p2.2) 
}


