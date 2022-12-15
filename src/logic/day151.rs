use std::cmp::{max, min};

use crate::read_input;

pub struct Day151;
impl Day151 {
    pub fn run(&self) {
        let filename = "input/day151.txt";
        let input = read_input(filename);

        let mut sensors = Vec::new();
        let mut beacons = Vec::new();

        // Need to store the smallest and largest x and y
        let mut minx = 0;
        let mut maxx = 0;
        let mut miny = 0;
        let mut maxy = 0;

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
            // println!("Distance found: {}", distance);
            sensors.push((sensor_xy[0], sensor_xy[1], distance));

            minx = min(minx, min(sensor_xy[0]-distance, beacon_xy[0]));
            maxx = max(maxx, max(sensor_xy[0]+distance, beacon_xy[0]));
            miny = min(miny, min(sensor_xy[1], beacon_xy[1]));
            maxy = max(maxy, max(sensor_xy[1], beacon_xy[1]));
        }

        let x_add_with = 0;
        let x_corr = -minx + x_add_with / 2;
        let y_corr = -miny;

        let row = 2000000;
        let mut target_row = Vec::new();

        // Build the empty target row
        for _x in 0..=maxx + x_corr {
            target_row.push('.');
        }

        // Add the sensors
        for s in sensors {
            // println!("Processing Sensor: {},{} Distance: {}", s.0, s.1, s.2);
            let mut y = -s.2;
            loop {
                // Ignore anything beyond the target row
                let effective_y = s.1 + y + y_corr;
                if effective_y == row {
                    let x = s.2 - y.abs();
                    for x in -x..=x {
                        target_row[(s.0 + x + x_corr) as usize] = '#';
                    }
                }
                if y == s.2 {
                    break;
                }
                y += 1;
            }
            // Mark the signal position
            if s.1 + y_corr == row {
                target_row[(s.0 + x_corr) as usize] = 'S';
            }
        }

        // Add the beacons (as these should not be counted as impossible positions)
        for b in beacons {
            if b.1 + y_corr == row {
                target_row[(b.0 + x_corr) as usize] = 'B';
            }
        }

        // Count the number of positions a beacon cannot exist
        let mut impossible_positions = 0;
        for c in target_row {
            if c == '#' || c=='S' {
                impossible_positions += 1;
            }
        }

        // Print the result
        println!("Day 15 - Part 1: Result is {}", impossible_positions);
    }
}
