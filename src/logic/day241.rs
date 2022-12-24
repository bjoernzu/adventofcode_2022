use std::collections::{HashSet, VecDeque};

use crate::logic::Puzzle;
use crate::read_input;

const DEBUG: bool = false;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

const DIRECTION_CHARS: [char; 4] = ['^', 'v', '<', '>'];
const DIRECTIONS: [Position; 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

type Position = (i32, i32);

fn draw_valley(elves: &Position, blizzards: &[Vec<Vec<bool>>; 4]) {
    let height = blizzards[UP].len();
    let width = blizzards[UP][0].len();

    for y in 0..height {
        for x in 0..width {
            if elves.0 == x as i32 && elves.1 == y as i32 {
                print!("E");
            } else {
                let num: i32 = blizzards.iter().map(|b| if b[y][x] { 1 } else { 0 }).sum();
                if num > 1 {
                    print!("{}", num);
                } else {
                    let mut drawn = false;
                    for i in 0..4 {
                        if blizzards[i][y][x] {
                            print! {"{}", DIRECTION_CHARS[i]};
                            drawn = true;
                        }
                    }
                    if !drawn {
                        print!(".");
                    }
                }
            }
        }
        print!("\n")
    }
}

fn move_blizzards(blizzards: &mut [Vec<Vec<bool>>; 4]) {
    let first = blizzards[UP].remove(0);
    blizzards[UP].push(first);

    let last = blizzards[DOWN].pop().unwrap();
    blizzards[DOWN].insert(0, last);

    let height = blizzards[UP].len();

    for i in 0..height {
        let left = blizzards[LEFT][i].remove(0);
        blizzards[LEFT][i].push(left);
        let right = blizzards[RIGHT][i].pop().unwrap();
        blizzards[RIGHT][i].insert(0, right);
    }
}

pub struct Day241;
impl Puzzle for Day241 {
    fn run(&self) {
        let filename = "input/day241.txt";
        let input = read_input(filename);

        let mut blizzards: [Vec<Vec<bool>>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut target_position: Position = (0, 0);
        // Parse the input
        for line in input.lines() {
            if line.starts_with("#.#") {
                // Starting row
            } else if line.starts_with("##") {
                target_position.1 = (blizzards[0].len()) as i32;
                target_position.0 = (blizzards[0][0].len() - 1) as i32;
                // Last row
            } else if !line.is_empty() {
                let mut up = Vec::new();
                let mut down = Vec::new();
                let mut left = Vec::new();
                let mut right = Vec::new();
                // Normal valley row
                for c in line.chars() {
                    match c {
                        '.' => {
                            up.push(false);
                            down.push(false);
                            left.push(false);
                            right.push(false)
                        }
                        '^' => {
                            up.push(true);
                            down.push(false);
                            left.push(false);
                            right.push(false)
                        }
                        'v' => {
                            up.push(false);
                            down.push(true);
                            left.push(false);
                            right.push(false)
                        }
                        '<' => {
                            up.push(false);
                            down.push(false);
                            left.push(true);
                            right.push(false)
                        }
                        '>' => {
                            up.push(false);
                            down.push(false);
                            left.push(false);
                            right.push(true)
                        }
                        _ => {}
                    }
                }
                blizzards[UP].push(up);
                blizzards[DOWN].push(down);
                blizzards[LEFT].push(left);
                blizzards[RIGHT].push(right);
            }
        }

        // Simulate Blizzard positions
        let mut blizzard_positions: Vec<HashSet<Position>> = Vec::new();
        let mut blizzard_loop_length = 0;
        let height = blizzards[UP].len();
        let width = blizzards[UP][0].len();
        loop {
            let mut new_blizzard = HashSet::new();
            for y in 0..height {
                for x in 0..width {
                    for b in &blizzards {
                        if b[y][x] {
                            new_blizzard.insert((x as i32, y as i32));
                        }
                    }
                }
            }
            blizzard_positions.push(new_blizzard.clone());
            if blizzard_loop_length != 0 && new_blizzard == blizzard_positions[0] {
                break;
            }
            blizzard_loop_length += 1;
            move_blizzards(&mut blizzards);
        }

        let starting_position: Position = (0, -1);

        if DEBUG {
            draw_valley(&starting_position, &blizzards);
            println!(
                "Blizzard pattern loops every {} minutes",
                blizzard_loop_length
            );
        }

        // Find the number
        let mut result = 0;
        let mut open_positions: VecDeque<(Position, usize)> = VecDeque::new();
        open_positions.push_back((starting_position, 0));

        let mut closed_positions: HashSet<(Position, usize)> = HashSet::new();

        let mut found = false;
        while !open_positions.is_empty() && !found {
            let (current_position, minute) = open_positions.pop_front().unwrap();
            let blizzard_pattern = (minute + 1) % blizzard_loop_length;
 
            
            // Check in which direction we can move
            for d in DIRECTIONS {
                let new_elves_pos = (current_position.0 + d.0, current_position.1 + d.1);

                if new_elves_pos == target_position {
                    result = minute + 1;
                    found = true;
                    break;
                }

                if new_elves_pos == starting_position
                    || (new_elves_pos.0 >= 0
                        && new_elves_pos.0 < width as i32
                        && new_elves_pos.1 >= 0
                        && new_elves_pos.1 < height as i32)
                        && !blizzard_positions[blizzard_pattern]
                            .contains(&(new_elves_pos))
                        && !closed_positions.contains(&(new_elves_pos, blizzard_pattern))
                {
                    open_positions.push_back((new_elves_pos, minute + 1));
                    closed_positions.insert((new_elves_pos, minute + 1));
                }
            }
        }

        println!("Day 24 - Part 1: Result is {}", result);
    }
}
