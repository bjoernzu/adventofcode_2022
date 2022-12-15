use std::cmp::{max, min};

use crate::read_input;

struct Sensor {
    x: i64,
    y: i64,
    distance: i64
}



pub struct Day152;
impl Day152 {
    pub fn run(&self) {
        let filename = "input/day152.txt";
        let input = read_input(filename);

        let mut sensors = Vec::new();
        let mut beacons = Vec::new();

        // Need to store the smallest and largest x and y
        let minx = 0;
        let maxx = 4000000;
        let miny = 0;
        let maxy = 4000000;

        for line in input.lines() {
            let parts: Vec<&str> = line.split(": closest beacon is at x=").collect();

            let beacon_xy: Vec<i64> = parts[1].split(", y=").map(|s| s.parse().unwrap()).collect();
            beacons.push((beacon_xy[0], beacon_xy[1]));

            let sensor_parts: Vec<&str> = parts[0].split("Sensor at x=").collect();
            let sensor_xy: Vec<i64> = sensor_parts[1]
                .split(", y=")
                .map(|s| s.parse().unwrap())
                .collect();
            let distance =
                (beacon_xy[0] - sensor_xy[0]).abs() + (beacon_xy[1] - sensor_xy[1]).abs();
            sensors.push((sensor_xy[0], sensor_xy[1], distance));

        }

        // Build the empty cave
        for y in miny..=maxy {
            for x in minx..=maxx {
                if !sensors.iter().any(|s| (s.0-x).abs() + (s.1-y).abs() <= s.2) {
                    // Print the result
                    let result = x * 4000000 + y;
                    println!("Day 15 - Part 2: Result is {}", result);
                    return
                }
            }
        }

    }
}

