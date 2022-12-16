use std::{cmp::{max, min}};

use crate::read_input;

struct Sensor {
    x: i64,
    y: i64,
    distance: i64
}

impl Sensor {
    fn new (x: i64,y: i64,distance: i64) -> Sensor {
        Sensor {
            x,
            y,
            distance
        }
    }

    fn range_at_y(&self, y: i64, minx: i64, maxx: i64) -> (i64, i64) {
        if (self.y - y).abs() > self.distance {
            return (0,0); // If the empty spot is at the first position of a line returninh (0,0) will fail the whole algorithm
        }
        let x_diff = self.distance - (self.y - y).abs();
        let range = (max(minx, self.x - x_diff), min(maxx, self.x + x_diff));
        return range;
    }
}

pub struct Day152;
impl Day152 {
    pub fn run(&self) {
        let filename = "input/day152.txt";
        let input = read_input(filename);

        let mut sensors = Vec::new();

        // Need to store the smallest and largest x and y
        let minx = 0;
        let maxx = 4000000;
        let miny = 0;
        let maxy = 4000000;

        for line in input.lines() {
            let parts: Vec<&str> = line.split(": closest beacon is at x=").collect();

            let beacon_xy: Vec<i64> = parts[1].split(", y=").map(|s| s.parse().unwrap()).collect();

            let sensor_parts: Vec<&str> = parts[0].split("Sensor at x=").collect();
            let sensor_xy: Vec<i64> = sensor_parts[1]
                .split(", y=")
                .map(|s| s.parse().unwrap())
                .collect();
            let distance =
                (beacon_xy[0] - sensor_xy[0]).abs() + (beacon_xy[1] - sensor_xy[1]).abs();
            sensors.push(Sensor::new(sensor_xy[0], sensor_xy[1], distance));

        }

        // Build the empty cave
        for y in miny..=maxy {
            let mut ranges: Vec<(i64, i64)> = sensors.iter().map(|s| s.range_at_y(y, minx, maxx)).collect();
            ranges.retain(|&r| r != (0,0));
            ranges.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
            let mut coverage = ranges.remove(0);
            
            for range in ranges {
                if overlap(&range, &coverage) {
                    // println!("{},{} and {}, {} overlap", range.0, range.1, coverage.0, coverage.1);
                    coverage.1 = max(coverage.1,range.1);
                }
                else {
                    println!("{},{} and {}, {} do not overlap in line {}", range.0, range.1, coverage.0, coverage.1, y);
                    // Print the result
                    let result = (coverage.1 + 1) * 4000000 + y;
                    println!("Day 15 - Part 2: Result is {}", result);
                    return
                }
            }
        }

    }
}

fn overlap(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    !(r1.0 > r2.1 || r1.1 < r2.0)
}