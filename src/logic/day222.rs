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
const RIGHT: Movement = DIRECTIONS[0];
const DOWN: Movement = DIRECTIONS[1];
const LEFT: Movement = DIRECTIONS[2];
const UP: Movement = DIRECTIONS[3];

fn turn(current_movement: &Movement, turn: char) -> Movement {
    // print!("Turn {} from direction {}, {} ", turn, current_movement.0, current_movement.1);
    let current_position = DIRECTIONS
        .iter()
        .position(|d| d == current_movement)
        .unwrap();
    let new_direction = match turn {
        'R' => DIRECTIONS[(current_position + 1) % 4],
        'L' => DIRECTIONS[(current_position + 4 - 1) % 4],
        _ => DIRECTIONS[current_position],
    };
    // println!("New direction: {}, {}", new_direction.0, new_direction.1);
    return new_direction;
}

fn move_it(
    current_position: &Position,
    moves: i32,
    direction: &Movement,
    map: &HashMap<Position, bool>,
) -> (Position, Movement) {
    let mut final_position = current_position.clone();
    let mut final_direction = direction.clone();
    let mut next_direction = direction.clone();
    for _i in 0..moves {
        let mut next_position = (
            final_position.0 + final_direction.0,
            final_position.1 + final_direction.1,
        );
        if !map.contains_key(&next_position) {
            (next_position, next_direction) =
                move_around_edge(&final_position, &final_direction);
        }
        // println!("Next position: {}, {}", next_position.0, next_position.1);
        if !map.get(&next_position).unwrap() {
            final_position = next_position;
            final_direction = next_direction;
        } else {
            return (final_position, final_direction);
        }
    }
    return (final_position, final_direction);
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

pub struct Day222;
impl Puzzle for Day222 {
    fn run(&self) {
        let filename = "input/day222.txt";
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

        let mut current_position: Position = find_first_in_row(&map, 1);
        // println!("Starting at {}, {}", current_position.0, current_position.1);
        let mut current_movement: Movement = (0, 1);
        for ops in &operations {
            match ops {
                Operation::Move(o) => {
                    (current_position, current_movement) =
                        move_it(&current_position, *o, &current_movement, &map)
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

        println!("Day 22 - Part 2: Result is {}", result);
    }
}

fn move_around_edge(
    current_position: &Position,
    direction: &Movement,
) -> (Position, Movement) {
    // Example faces
    // let edge_len = 4;
    // let faces: [Position; 6] = [(1, 9), (5, 1), (5, 5), (5, 9), (9, 9), (9, 13)];

    // Challenge faces
    let edge_len = 50;
    let faces: [Position; 6] = [(1, 51), (1, 101), (51, 51), (101, 1), (101, 51), (151, 1)];

    // Find current face
    let mut current_face = 0;
    for face in faces {
        current_face += 1;
        if current_position.0 >= face.0
            && current_position.0 < face.0 + edge_len
            && current_position.1 >= face.1
            && current_position.1 < face.1 + edge_len
        {
            break;
        }
    }

    let mut edges: HashMap<(i32, Movement), (i32, Movement)> = HashMap::new();

    // Example Edges
    // // From face 1
    // edges.insert((1, LEFT), (3, DOWN)); // Leaving 1 on the left, got to 3
    // edges.insert((1, UP), (2, DOWN)); // Leaving 1 on the top, go to 2
    // edges.insert((1, RIGHT), (6, LEFT)); // Leaving 1 on the right, go to 6
    // // From face 2
    // edges.insert((2, UP), (1, DOWN));
    // edges.insert((2, LEFT), (6, UP));
    // edges.insert((2, DOWN), (5, UP));
    // // From face 3
    // edges.insert((3, UP), (1, RIGHT));
    // edges.insert((3, DOWN), (5, RIGHT));
    // // From face 4
    // edges.insert((4, RIGHT), (6, DOWN));
    // // From face 5
    // edges.insert((5, LEFT), (3, UP));
    // edges.insert((5, DOWN), (2, UP));
    // // From face 6
    // edges.insert((6, UP), (4, LEFT));
    // edges.insert((6, RIGHT), (1, LEFT));
    // edges.insert((6, DOWN), (2, RIGHT));

    // Challenge Edges
    // From face 1
    edges.insert((1, LEFT), (4, RIGHT));
    edges.insert((1, UP), (6, RIGHT));
    // From face 2
    edges.insert((2, UP), (6, UP));
    edges.insert((2, RIGHT), (5, LEFT));
    edges.insert((2, DOWN), (3, LEFT));
    // From face 3
    edges.insert((3, RIGHT), (2, UP));
    edges.insert((3, LEFT), (4, DOWN));
    // From face 4
    edges.insert((4, UP), (3, RIGHT));
    edges.insert((4, LEFT), (1, RIGHT));
    // From face 5
    edges.insert((5, DOWN), (6, LEFT));
    edges.insert((5, RIGHT), (2, LEFT));
    // From face 6
    edges.insert((6, LEFT), (1, DOWN));
    edges.insert((6, RIGHT), (5, UP));
    edges.insert((6, DOWN), (2, DOWN));

    let edge = edges.get(&(current_face, *direction)).unwrap();
    let current_face_pos = faces[current_face as usize - 1];
    let new_face = edge.0;
    let new_face_pos = faces[new_face as usize - 1];
    let new_direction = edge.1;

    // Position mapping
    let relative_row = current_position.0 - current_face_pos.0;
    let relative_column = current_position.1 - current_face_pos.1;
    let end_offset = edge_len - 1;

    let new_position = match (*direction, new_direction) {
        // From Down
        (DOWN, RIGHT) => (
            new_face_pos.0 + end_offset - relative_row,
            current_face_pos.1,
        ),
        (DOWN, DOWN) => (new_face_pos.0, new_face_pos.1 + relative_column),
        (DOWN, LEFT) => (
            new_face_pos.0 + relative_column,
            new_face_pos.1 + end_offset,
        ),
        (DOWN, UP) => (
            new_face_pos.0 + end_offset,
            new_face_pos.1 + end_offset - relative_column,
        ),
        // From Right
        (RIGHT, DOWN) => (new_face_pos.0, new_face_pos.1 + end_offset - relative_row),
        (RIGHT, LEFT) => (
            new_face_pos.0 + end_offset - relative_row,
            new_face_pos.1 + end_offset,
        ),
        (RIGHT, UP) => (new_face_pos.0 + end_offset, new_face_pos.1 + relative_row),
        // From Left
        (LEFT, DOWN) => (new_face_pos.0, new_face_pos.1 + relative_row),
        (LEFT, RIGHT) => (new_face_pos.0 + end_offset - relative_row, new_face_pos.1),
        (LEFT, UP) => (
            new_face_pos.0 + end_offset,
            new_face_pos.1 + end_offset - relative_row,
        ),
        // From UP
        (UP, RIGHT) => (new_face_pos.0 + relative_column, new_face_pos.1),
        (UP, DOWN) => (
            new_face_pos.0,
            new_face_pos.1 + end_offset - relative_column,
        ),
        (UP, LEFT) => (
            new_face_pos.0 + end_offset - relative_row,
            new_face_pos.1 + end_offset,
        ),
        (UP, UP) => (
            new_face_pos.0 + end_offset,
            new_face_pos.1 + relative_column,
        ),

        _ => {
            println!(
                "Unknown combination {}, {} to {}, {}",
                direction.0, direction.1, new_direction.0, new_direction.1
            );
            (0, 0)
        }
    };
    let validated = [
        (DOWN, LEFT),
        (RIGHT, LEFT),
        (UP, UP),
        (RIGHT, UP),
        (DOWN, DOWN),
        (LEFT, RIGHT),
        (LEFT, DOWN),
        (UP, RIGHT),
    ];

    if !validated.contains(&(*direction, new_direction)) {
        println!("Going around edge from face {} ({},{}) with direction {},{} to edge {} ({},{} with new direction {},{})", current_face, current_position.0, current_position.1, direction.0, direction.1, new_face, new_position.0, new_position.1, new_direction.0, new_direction.1);
    }

    return (new_position, new_direction);
}
