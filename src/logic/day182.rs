use std::collections::HashSet;

use crate::logic::Puzzle;
use crate::read_input;

type Pixel = (i8, i8, i8);

pub struct Day182;
impl Puzzle for Day182 {
    fn run(&self) {
        let filename = "input/day182.txt";
        let input = read_input(filename);

        let mut lava_pixels: HashSet<Pixel> = HashSet::new();
        let mut water_pixels: Box<HashSet<Pixel>> = Box::new(HashSet::new());

        let mut min: Pixel = (0,0,0);
        let mut max: Pixel = (0,0,0);

        let mut lines: Vec<&str> = input.lines().collect();
        lines.sort();

        // Parse all the pixels
        for line in lines {
            if !line.is_empty() {
                let dots: Vec<i8> = line.split(",").map(|c| c.parse::<i8>().unwrap()).collect();
                let pixel: Pixel = (dots[0], dots[1], dots[2]);
                lava_pixels.insert(pixel);
            }
        }

        // Determine the volume we need to fill
        min.0 = lava_pixels.iter().map(|p| p.0).min().unwrap() - 1;
        min.1 = lava_pixels.iter().map(|p| p.1).min().unwrap() - 1;
        min.2 = lava_pixels.iter().map(|p| p.2).min().unwrap() - 1;
        max.0 = lava_pixels.iter().map(|p| p.0).max().unwrap() + 1;
        max.1 = lava_pixels.iter().map(|p| p.1).max().unwrap() + 1;
        max.2 = lava_pixels.iter().map(|p| p.2).max().unwrap() + 1;

        let mut surface_area = 0;

        // Set the source for our water
        water_pixels.insert(min);

        let mut queue = Vec::new();
        queue.push(min);

        while !queue.is_empty() {
            
            let p = queue.remove(0);
            // println!("Filling with water: {},{},{}", p.0, p.1, p.2);
                if lava_pixels.contains(&(p.0-1,p.1,p.2)) {surface_area += 1;} else if p.0 - 1 >= min.0 && p.0 - 1 <= max.0 && !water_pixels.contains(&(p.0-1,p.1,p.2)) {water_pixels.insert((p.0-1,p.1,p.2)); queue.push((p.0-1,p.1,p.2));}
                if lava_pixels.contains(&(p.0+1,p.1,p.2)) {surface_area += 1;} else if p.0 + 1 >= min.0 && p.0 + 1 <= max.0 && !water_pixels.contains(&(p.0+1,p.1,p.2)) {water_pixels.insert((p.0+1,p.1,p.2)); queue.push((p.0+1,p.1,p.2));}
                if lava_pixels.contains(&(p.0,p.1-1,p.2)) {surface_area += 1;} else if p.1 - 1 >= min.1 && p.1 - 1 <= max.1 && !water_pixels.contains(&(p.0,p.1-1,p.2)) {water_pixels.insert((p.0,p.1-1,p.2)); queue.push((p.0,p.1-1,p.2));}
                if lava_pixels.contains(&(p.0,p.1+1,p.2)) {surface_area += 1;} else if p.1 + 1 >= min.1 && p.1 + 1 <= max.1 && !water_pixels.contains(&(p.0,p.1+1,p.2)) {water_pixels.insert((p.0,p.1+1,p.2)); queue.push((p.0,p.1+1,p.2));}
                if lava_pixels.contains(&(p.0,p.1,p.2-1)) {surface_area += 1;} else if p.2 - 1 >= min.2 && p.2 - 1 <= max.2 && !water_pixels.contains(&(p.0,p.1,p.2-1)) {water_pixels.insert((p.0,p.1,p.2-1)); queue.push((p.0,p.1,p.2-1));}
                if lava_pixels.contains(&(p.0,p.1,p.2+1)) {surface_area += 1;} else if p.2 + 1 >= min.2 && p.2 - 1 <= max.2 && !water_pixels.contains(&(p.0,p.1,p.2+1)) {water_pixels.insert((p.0,p.1,p.2+1)); queue.push((p.0,p.1,p.2+1));}
        }


        println!("Day 18 - Part 2: Result is {}", surface_area);
    }
}