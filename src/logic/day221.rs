use regex::Regex;
use std::collections::HashMap;

use crate::logic::Puzzle;
use crate::read_input;

type Position = (i32, i32);
type Movement = (i32, i32);
enum Operation {
    Move(i32),
    Turn(char),
}

const DIRECTIONS: [Movement; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn turn(current_movement: &Movement, turn: char) -> Movement {
    // print!("Turn {} from direction {}, {} ", turn, current_movement.0, current_movement.1);
    let current_position = DIRECTIONS
        .iter()
        .position(|d| d == current_movement)
        .unwrap();
    let new_direction = match turn {
        'R' =>  DIRECTIONS[(current_position + 1) % 4],
        'L' =>  DIRECTIONS[(current_position + 4 - 1) % 4],
        _ =>  DIRECTIONS[current_position],
    };
    // println!("New direction: {}, {}", new_direction.0, new_direction.1);
    return new_direction
}

fn move_it(
    current_position: &Position,
    moves: i32,
    direction: &Movement,
    map: &HashMap<Position, bool>,
) -> Position {
    // println!("Move {} into direction {}, {} from position {}, {} ", moves, direction.0, direction.1, current_position.0, current_position.1);
    let mut final_position = current_position.clone();
    for _i in 0..moves {
        let mut next_position = (
            final_position.0 + direction.0,
            final_position.1 + direction.1,
        );
        if !map.contains_key(&next_position) {
            next_position = wrap(&final_position, direction, map);
        }
        if !map.get(&next_position).unwrap() {
            final_position = next_position;
        } else {
            // println!("Moved to position {},{} due to block", final_position.0, final_position.1);
            return final_position;
        }
    }
    // println!("Moved to position {},{}", final_position.0, final_position.1);
    return final_position;
}

fn wrap(
    current_position: &Position,
    direction: &Movement,
    map: &HashMap<Position, bool>,
) -> Position {
    match direction {
        (0, 1) => find_first_in_row(map, current_position.0),
        (1, 0) => find_first_in_col(map, current_position.1),
        (0, -1) => find_last_in_row(map, current_position.0),
        (-1, 0) => find_last_in_col(map, current_position.1),
        _ => *current_position,
    }
}

fn find_first_in_row(map: &HashMap<Position, bool>, row: i32) -> Position {
    let mut col: i32 = 1;
    loop {
        // println!("Checking for {}, {}", row, col);
        if map.contains_key(&(row, col)) {
            return (row, col);
        }
        col += 1;
    }
}

fn find_last_in_row(map: &HashMap<Position, bool>, row: i32) -> Position {
    let mut col: i32 = 1;
    let mut found_first = false;
    loop {
        // println!("Checking for {}, {}", row, col);
        if found_first && !map.contains_key(&(row, col)) {
            return (row, col - 1);
        }
        if map.contains_key(&(row, col)) {
            found_first = true;
        }
        col += 1;
    }
}

fn find_first_in_col(map: &HashMap<Position, bool>, col: i32) -> Position {
    let mut row: i32 = 1;
    loop {
        // println!("Checking for {}, {}", row, col);
        if map.contains_key(&(row, col)) {
            return (row, col);
        }
        row += 1;
    }
}

fn find_last_in_col(map: &HashMap<Position, bool>, col: i32) -> Position {
    let mut row: i32 = 1;
    let mut found_first = false;
    loop {
        // println!("Checking for {}, {}", row, col);
        if found_first && !map.contains_key(&(row, col)) {
            return (row - 1, col);
        }
        if map.contains_key(&(row, col)) {
            found_first = true;
        }
        row += 1;
    }
}

pub struct Day221;
impl Puzzle for Day221 {
    fn run(&self) {
        let filename = "input/day221.txt";
        let input = read_input(filename);

        let mut map: HashMap<Position, bool> = HashMap::new();

        let mut map_lines: bool = true;
        let mut map_position: Position = (1, 1);

        let mut operations: Vec<Operation> = Vec::new();

        // Parse the input
        for line in input.lines() {
            if !line.is_empty() {
                if map_lines {
                    map_position.1 = 1;
                    for c in line.chars() {
                        match c {
                            '.' => map.insert(map_position, false),
                            '#' => map.insert(map_position, true),
                            _ => Some(false),
                        };
                        map_position.1 += 1;
                    }
                    map_position.0 += 1;
                } else {
                    let re = Regex::new(r"\d+|[RL]").unwrap();
                    operations = re
                        .captures_iter(line)
                        .map(|c| match &c[0] {
                            "R" => Operation::Turn(c[0].chars().nth(0).unwrap()),
                            "L" => Operation::Turn(c[0].chars().nth(0).unwrap()),
                            _ => Operation::Move(c[0].parse::<i32>().unwrap()),
                        })
                        .collect();
                }
            } else {
                map_lines = false;
            }
        }

        // for row in 1..20 {
        //     for col in 1..20 {
        //         if map.contains_key(&(row, col)) {
        //             if *map.get(&(row, col)).unwrap() {
        //                 print!("#");
        //             }
        //             else {print!(".")}
                    
        //         }
        //         else {
        //             print!(" ");
        //         }
        //     }
        //     print!("\n");
        // }
        
        let mut current_position: Position = find_first_in_row(&map, 1);
        // println!("Starting at {}, {}", current_position.0, current_position.1);
        let mut current_movement: Movement = (0, 1);
        for ops in &operations {
            match ops {
                Operation::Move(o) => {
                    current_position = move_it(&current_position, *o, &current_movement, &map)
                }
                Operation::Turn(o) => current_movement = turn(&current_movement, *o),
            }
        }

        let result = current_position.0 * 1000
            + current_position.1 * 4
            + DIRECTIONS
                .iter()
                .position(|d| d == &current_movement)
                .unwrap() as i32;

        println!("Day 22 - Part 1: Result is {}", result);
    }
}
