use crate::read_input;
use crate::logic::Puzzle;use std::cmp::{max, min};
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct Day121;
impl Puzzle for Day121 {
    fn run(&self) {
        let filename = "input/day121.txt";
        let input = read_input(filename);

        let mut map: Vec<Vec<u32>> = Vec::new();

        for line in input.lines() {
            if !line.is_empty() {
                map.push(line.chars().map(|c| c as u32).collect());
            }
        }

        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);

        // Find start and end position
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                if map[x][y] == 'S' as u32 {
                    start = (x, y);
                    map[x][y] = 96;
                } else if map[x][y] == 'E' as u32 {
                    end = (x, y);
                    map[x][y] = 122;
                }
            }
        }

        let result = find_way(&start, &end, &map);
        // Print the result
        println!("Day 12 - Part 1: Result is {}", result.last().unwrap().g_score);
    }
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
    height: u32,
    g_score: usize,
    f_score: usize,
}

impl Point {
    fn new(x: usize, y: usize, map: &Vec<Vec<u32>>) -> Point {
        Point {
            x,
            y,
            height: map[x][y],
            g_score: usize::MAX,
            f_score: usize::MAX,
        }
    }

    fn calculate_distance(&mut self, target: &Point) -> usize {
        let distance = ((self.x as i64 - target.x as i64).pow(2) as f64
            + (self.y as i64 - target.y as i64).pow(2) as f64)
            .sqrt() as usize;
        let high_distance = target.height - self.height;
        min(distance, high_distance as usize)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn construct_path(
    came_from_list: &mut HashMap<Rc<Point>, Rc<Point>>,
    last: Rc<Point>,
) -> Vec<Rc<Point>> {
    let mut path = Vec::new();
    let mut current = last;
    while came_from_list.contains_key(&current) {
        path.push(current.clone());
        current = came_from_list.remove(&current).unwrap();
    }
    path.reverse();
    return path;
}

// A* algorithm to find the shortest way
fn find_way(
    start: &(usize, usize),
    target: &(usize, usize),
    map: &Vec<Vec<u32>>,
) -> Vec<Rc<Point>> {
    let target = Point::new(target.0, target.1, map);
    let mut start = Point::new(start.0, start.1, map);
    start.g_score = 0;
    start.f_score = start.calculate_distance(&target);
    let start_p = Rc::new(start);

    // I need a place to store all the points
    let mut points: Vec<Rc<Point>> = Vec::new();

    let mut open_list: Vec<Rc<Point>> = Vec::new();
    open_list.push(start_p.clone());

    let mut closed_list: Vec<Rc<Point>> = Vec::new();
    closed_list.push(start_p.clone());

    let mut came_from_list: HashMap<Rc<Point>, Rc<Point>> = HashMap::new();

    while !open_list.is_empty() {
        // Find best next option
        open_list.sort();
        let current: Rc<Point> = open_list.remove(0);
        closed_list.push(current.clone());

        // Check if we are at the goal
        if current.x == target.x && current.y == target.y {
            return construct_path(&mut came_from_list, current);
        }

        // Add possible options to open_list
        let xmax = map.len() - 1;
        let ymax = map[0].len() - 1;
        let neighbors = vec![
            Point::new(current.x, min(current.y + 1, ymax), map),
            Point::new(current.x, max(current.y as i64 - 1, 0) as usize, map),
            Point::new(min(current.x + 1, xmax), current.y, map),
            Point::new(max(current.x as i64 - 1, 0) as usize, current.y, map),
        ];

        for mut n in neighbors {
            let tentative_g_score = current.g_score + 1;
            if tentative_g_score < n.g_score && n.height <= current.height + 1 {
                n.g_score = tentative_g_score;
                n.f_score = tentative_g_score + n.calculate_distance(&target);
                let pn = Rc::new(n);
                points.push(pn.clone());
                came_from_list.insert(pn.clone(), current.clone());
                if !open_list.contains(&pn) && !closed_list.contains(&pn) {
                    open_list.push(pn.clone())
                }
            }
        }
    }

    let path = Vec::new();
    return path;
}

// Dump and inefficient way to find the shortest way
// Works for the example, but not for the real input
fn _find_way_stupid(
    current: &(usize, usize),
    visited: &Vec<(usize, usize)>,
    target: &(usize, usize),
    map: &Vec<Vec<u32>>,
) -> (usize, Vec<(usize, usize)>) {
    // println!("Visiting node {}, {}: {}", current.0, current.1, map[current.0][current.1]);
    if current == target {
        // println!("Found target: {}, {}: {}", current.0, current.1, map[current.0][current.1]);
        return (0, vec![*current]);
    } else {
        // Define the possible directions
        let xmax = map.len() - 1;
        let ymax = map[0].len() - 1;
        let directions = vec![
            (current.0, min(current.1 + 1, ymax)),
            (current.0, max(current.1 as i64 - 1, 0) as usize),
            (min(current.0 + 1, xmax), current.1),
            (max(current.0 as i64 - 1, 0) as usize, current.1),
        ];

        // Add current node to the visited notes
        let mut path = visited.clone();
        path.push(*current);

        // Try possible directions and check which has the lowest costs
        let mut min_path = (99999999, Vec::new());
        for d in directions {
            // Consider option only, if the new node is not in the visited nodes yet and if value is max 1 higher than current
            if !path.contains(&d) && map[d.0][d.1] <= map[current.0][current.1] + 1 {
                let dir_path = _find_way_stupid(&d, &path, target, map);
                if dir_path.0 < min_path.0 {
                    min_path = dir_path;
                }
            }
        }
        min_path.1.push(*current);
        return (min_path.0 + 1, min_path.1);
    }
}
