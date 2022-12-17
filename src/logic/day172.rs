use std::{cmp::max, collections::HashMap};

use crate::read_input;

type Shape = Vec<Vec<bool>>;
type Chamber = Vec<Vec<bool>>;
struct Stone {
    shape: Shape,
    x: usize,
    y: usize,
    height: usize,
    width: usize,
}

impl Stone {
    fn new(shape: Shape, x: usize, y: usize) -> Stone {
        let height = shape.len();
        let width = shape[0].len();
        Stone {
            shape,
            x,
            y,
            height,
            width,
        }
    }

    fn fall_down(&mut self, chamber: &Chamber) -> bool {
        if self.y == 0 {
            return false;
        }
        for yi in 0..self.height {
            // Ignore anything above the current chamber
            if self.y + yi < chamber.len() {
                for xi in 0..self.width {
                    if self.shape[yi][xi] {
                        if chamber[self.y + yi - 1][self.x + xi] {
                            return false;
                        }
                    }
                }
            }
        }
        self.y = self.y - 1;
        return true;
    }

    fn move_sideways(&mut self, x_move: i64, chamber: &Chamber) -> bool {
        // Cover special cases where we can directly return false
        if (self.x <= 0 && x_move == -1) || //Stone already on the left side and should be moved left
        (self.x + self.width >= chamber[0].len() && x_move == 1)
        // Stone already on the right side and should be moved right
        {
            return false;
        }

        for yi in 0..self.height {
            if self.y + yi < chamber.len() {
                // Ignore anything above the current chamber
                for xi in 0..self.width {
                    if self.shape[yi][xi] {
                        let newx = (self.x as i64 + x_move + xi as i64) as usize;
                        if chamber[self.y + yi][newx] {
                            return false;
                        }
                    }
                }
            }
        }
        self.x = (self.x as i64 + x_move) as usize;
        return true;
    }
}

pub struct Day172;
impl Day172 {
    pub fn run(&self) {
        let filename = "input/day172.txt";
        let input = read_input(filename);

        let jet: Vec<char> = input.chars().collect();
        let jet_length = input.len();
        let num_stones = 1000000000000;
        let window_size = 50; // I assume after 50 rows all y coordinates will be taken by at least one stone. Otherwise I would have to keep track and make the window dynamic.

        let mut cache: HashMap<
            (Chamber, usize, usize, usize),
            (Chamber, usize, usize, usize, usize, usize, usize),
        > = HashMap::new();

        let mut stone_shapes: Vec<Shape> = Vec::new();
        stone_shapes.push(vec![vec![true, true, true, true]]);
        stone_shapes.push(vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ]);
        stone_shapes.push(vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ]);
        stone_shapes.push(vec![vec![true], vec![true], vec![true], vec![true]]);
        stone_shapes.push(vec![vec![true, true], vec![true, true]]);

        // Build the initial chamber
        let mut chamber: Shape = Vec::new();
        let new_line = vec![false, false, false, false, false, false, false];
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());

        // Setup data to track globally
        let mut highest_point = 0;
        let mut deleted_rows = 0;

        let mut stone_i = 0;
        let mut current_stone: Stone = Stone::new(stone_shapes[0].clone(), 2, 3);
        let x = 2;

        let mut _solver_loops = 0;

        // Start the actual solver loop
        loop {
            // Describe the current iteration state
            let iteration_state = (
                chamber.clone(),
                stone_i % 5,
                current_stone.x,
                current_stone.y,
            );
            if _solver_loops % 100000 == 0 {
                println!(
                    "Solver loop: {}, Current height: {}, current stones: {}",
                    _solver_loops, highest_point, stone_i
                );
            }
            _solver_loops += 1;

            // Check if the current iteration state is already known
            if cache.contains_key(&iteration_state) {
                let cache_hit = cache.get(&iteration_state).unwrap();
                if cache_hit.6 + stone_i < num_stones {
                    chamber = cache_hit.0.clone();
                    current_stone =
                        Stone::new(stone_shapes[cache_hit.1].clone(), cache_hit.2, cache_hit.3);
                    highest_point += cache_hit.4;
                    deleted_rows += cache_hit.5;
                    stone_i += cache_hit.6;
                    continue;
                }
            }

            let iteration_start_stones = stone_i;
            let iteration_start_deleted_rows = deleted_rows;
            let iteration_start_highest_point = highest_point;

            for j in 0..jet_length {
                // print!("{}", jet[j]);
                match jet[j] {
                    '<' => current_stone.move_sideways(-1, &chamber),
                    '>' => current_stone.move_sideways(1, &chamber),
                    _ => false,
                };
                if !current_stone.fall_down(&chamber) {
                    // Add stone to chamber
                    for y in 0..current_stone.shape.len() {
                        for x in 0..current_stone.shape[y].len() {
                            if current_stone.shape[y][x] {
                                chamber[current_stone.y + y][current_stone.x + x] = true
                            }
                        }
                    }

                    highest_point = max(
                        highest_point,
                        current_stone.y + current_stone.height + deleted_rows,
                    );

                    for _i in 0..highest_point + 4 - chamber.len() - deleted_rows {
                        chamber.push(new_line.clone());
                    }
                    if chamber.len() > window_size {
                        deleted_rows += chamber.len() - window_size;
                        chamber.drain(..chamber.len() - window_size);
                    }
                    stone_i += 1;

                    current_stone = Stone::new(
                        stone_shapes[stone_i % 5].clone(),
                        x,
                        highest_point - deleted_rows + 3,
                    );

                    // Stop when target number of stones is reaches
                    if stone_i >= num_stones {
                        break;
                    }
                }
            }

            let target_iteration_state = (
                chamber.clone(),
                stone_i % 5,
                current_stone.x,
                current_stone.y,
            );
            
            // Check if we found a circle
            if iteration_state == target_iteration_state {
                let stones_left = num_stones - stone_i;
                // skip iterations
                let stones_this_iteration = stone_i - iteration_start_stones;
                let iterations = stones_left / stones_this_iteration as usize;
                stone_i += iterations * stones_this_iteration;
                highest_point += iterations * (highest_point - iteration_start_highest_point);
                deleted_rows += iterations * (deleted_rows - iteration_start_deleted_rows);
            }
            
            // Check if current target state has already been calculated and result can be concatenated
            if cache.contains_key(&target_iteration_state) {
                let cache_hit = cache.get(&target_iteration_state).unwrap();
                chamber = cache_hit.0.clone();
                current_stone =
                    Stone::new(stone_shapes[cache_hit.1].clone(), cache_hit.2, cache_hit.3);
                highest_point += cache_hit.4;
                deleted_rows += cache_hit.5;
                stone_i += cache_hit.6;
            }

            // Store state in cache

            // println!(
            //     "Shape: {}, x: {}, y: {}, height gain: {}, deleted rows: {}, stones {}",
            //     stone_i % 5,
            //     current_stone.x,
            //     current_stone.y,
            //     highest_point - iteration_start_highest_point,
            //     deleted_rows - iteration_start_deleted_rows,
            //     stone_i - iteration_start_stones,
            // );
            cache.insert(
                iteration_state,
                (
                    chamber.clone(),
                    stone_i % 5,
                    current_stone.x,
                    current_stone.y,
                    highest_point - iteration_start_highest_point,
                    deleted_rows - iteration_start_deleted_rows,
                    stone_i - iteration_start_stones,
                ),
            );

            // Stop when target number of stones is reaches
            if stone_i >= num_stones {
                break;
            }
        }
        // _draw_chamber(&chamber);

        let result = highest_point;

        println!("Day 17 - Part 2: Result is {}", result);
    }
}

fn _draw_chamber(chamber: &Shape) {
    let mut rev_chamber = chamber.clone();
    rev_chamber.reverse();
    for y in rev_chamber {
        print!("|");
        for x in y {
            if x {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("|\n");
    }
    println!("+-------+");
}

