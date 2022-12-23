use std::cmp::{max, min};
use std::collections::HashMap;

use crate::logic::Puzzle;
use crate::read_input;

const DEBUG: bool = false;

type Position = (i32, i32);
type Move = (i32, i32);

const NORTH: Move = (-1, 0);
const SOUTH: Move = (1, 0);
const WEST: Move = (0, -1);
const EAST: Move = (0, 1);
const NORTHWEST: Move = (-1, -1);
const NORTHEAST: Move = (-1, 1);
const SOUTHWEST: Move = (1, -1);
const SOUTHEAST: Move = (1, 1);
const MOVES: [Move; 4] = [NORTH, SOUTH, WEST, EAST];
const SURROUNDING: [Move; 8] = [
    NORTH, SOUTH, WEST, EAST, NORTHWEST, NORTHEAST, SOUTHEAST, SOUTHWEST,
];

fn apply_move(position: &Position, direction: &Move) -> Position {
    return (position.0 + direction.0, position.1 + direction.1);
}

fn get_bounding_box(elves: &HashMap<Position, bool>) -> (Position, Position) {
    let mut top_left: Position = (1, 1);
    let mut bottom_right: Position = (1, 1);

    for e in elves {
        top_left.0 = min(top_left.0, e.0 .0);
        top_left.1 = min(top_left.1, e.0 .1);
        bottom_right.0 = max(bottom_right.0, e.0 .0);
        bottom_right.1 = max(bottom_right.1, e.0 .1);
    }

    return (top_left, bottom_right);
}

fn draw_plan(elves: &HashMap<Position, bool>) {
    let bounding_box = get_bounding_box(elves);

    for row in bounding_box.0 .0..=bounding_box.1 .0 {
        for col in bounding_box.0 .1..=bounding_box.1 .1 {
            match elves.get(&(row, col)) {
                Some(_e) => print!("#"),
                _ => print!("."),
            };
        }
        print!("\n");
    }
}

pub struct Day232;
impl Puzzle for Day232 {
    fn run(&self) {
        let filename = "input/day232.txt";
        let input = read_input(filename);

        let mut elves: HashMap<Position, bool> = HashMap::new();
        let mut current_position: Position = (1, 1);
        // Parse the input
        for line in input.lines() {
            current_position.1 = 0;
            if !line.is_empty() {
                for c in line.chars() {
                    if c == '#' {
                        elves.insert(current_position, true);
                    }
                    current_position.1 += 1;
                }
            }
            current_position.0 += 1;
        }
        if DEBUG {
            draw_plan(&elves);
        }

        // Hardcode the checks
        let mut direction_checks: HashMap<Move, [Position; 3]> = HashMap::new();
        direction_checks.insert(NORTH, [NORTHWEST, NORTH, NORTHEAST]);
        direction_checks.insert(SOUTH, [SOUTHWEST, SOUTH, SOUTHEAST]);
        direction_checks.insert(WEST, [NORTHWEST, WEST, SOUTHWEST]);
        direction_checks.insert(EAST, [NORTHEAST, EAST, SOUTHEAST]);

        let mut starting_direction = 0;

        let mut moved = true;
        let mut rounds = 0;
        while moved {
            let mut new_positions: HashMap<Position, u8> = HashMap::new();
            let mut selected_direction: HashMap<Position, Move> = HashMap::new();
            moved = false;

            // Check where to move
            for e in &elves {
                if DEBUG {
                    println!("=== Checking elf {}, {} ===", e.0 .0, e.0 .1);
                }
                // Check if need to move at all
                let mut neighbours: HashMap<Move, bool> = HashMap::new();
                for s in SURROUNDING {
                    neighbours.insert(s, (&elves).contains_key(&apply_move(e.0, &s)));
                }

                // No need to move if there are no neighbors
                if neighbours.values().all(|n| !n) {
                    if DEBUG {
                        println!("Elf has no neighbors");
                    }
                    continue;
                }

                for m in 0..4 {
                    let direction = MOVES[(starting_direction + m) % 4];
                    if direction_checks
                        .get(&direction)
                        .unwrap()
                        .iter()
                        .all(|c| !neighbours.get(&c).unwrap())
                    {
                        *new_positions
                            .entry(apply_move(e.0, &direction))
                            .or_insert(0) += 1;
                        selected_direction.insert(e.0.clone(), direction);
                        moved = true;
                        if DEBUG {
                            println!(
                                "Elf can move into direction {}, {}",
                                direction.0, direction.1
                            );
                        }
                        break;
                    } else {
                        if DEBUG {
                            println!(
                                "Elf cannot move into direction {}, {}",
                                direction.0, direction.1
                            );
                        }
                    }
                }
            }

            // Perform moves
            let mut new_elves = HashMap::new();
            for e in &elves {
                if selected_direction.contains_key(e.0) {
                    let new_position = apply_move(e.0, selected_direction.get(e.0).unwrap());
                    match new_positions.get(&new_position) {
                        Some(n) => {
                            if n < &(2 as u8) {
                                new_elves.insert(new_position, true);
                            } else {
                                new_elves.insert(e.0.clone(), true);
                            }
                        }
                        _ => {}
                    }
                } else {
                    new_elves.insert(e.0.clone(), true);
                }
            }
            elves = new_elves.clone();
            rounds += 1;
            // Shift starting move
            starting_direction += 1;
            if DEBUG {
                println!("Result after round {}", rounds);
                draw_plan(&elves);
            }
        }
        let result: i32 = rounds;

        println!("Day 23 - Part 2: Result is {}", result);
    }
}
