use std::cmp::max;

use crate::read_input;

struct Stone {
    shape: Vec<Vec<char>>,
    x: usize,
    y: usize,
    height: usize,
    width: usize,
}

impl Stone {
    fn new(shape: Vec<Vec<char>>, x: usize, y: usize) -> Stone {
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

    fn fall_down(&mut self, chamber: &Vec<Vec<char>>) -> bool {
        if self.y == 0 {
            return false;
        }
        for yi in 0..self.height {
            // Ignore anything above the current chamber
            if self.y + yi < chamber.len() {
                for xi in 0..self.width {
                    if self.shape[yi][xi] == '#' {
                        if chamber[self.y + yi - 1][self.x + xi] == '#' {
                            return false;
                        }
                    }
                }
            }
        }
        self.y = self.y - 1;
        return true;
    }

    fn move_sideways(&mut self, x_move: i64, chamber: &Vec<Vec<char>>) -> bool {
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
                    if self.shape[yi][xi] == '#' {
                        // println!(
                            //     "Checking position {}, {}",
                            //     self.y + yi,
                            //     self.x as i64 + x_move + xi as i64
                            // );
                            let newx = (self.x as i64 + x_move + xi as i64) as usize;
                            if chamber[self.y + yi][newx] == '#' {
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

pub struct Day171;
impl Day171 {
    pub fn run(&self) {
        let filename = "input/day171.txt";
        let input = read_input(filename);

        let mut flow = input.chars();
        let num_stones = 2022;

        let mut stone_shapes: Vec<Vec<Vec<char>>> = Vec::new();
        stone_shapes.push(vec![vec!['#', '#', '#', '#']]);
        stone_shapes.push(vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ]);
        stone_shapes.push(vec![
            vec!['#', '#', '#'],
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
        ]);
        stone_shapes.push(vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]);
        stone_shapes.push(vec![vec!['#', '#'], vec!['#', '#']]);

        // Build the initial chamber
        let mut chamber: Vec<Vec<char>> = Vec::new();
        let new_line = vec!['.', '.', '.', '.', '.', '.', '.'];
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());
        chamber.push(new_line.clone());

        let mut highest_point = 0;

        for i in 0..num_stones {
            let mut stone = Stone::new(stone_shapes[i % 5].clone(), 2, highest_point + 3);

            loop {
                // Check impact of jet
                let jet_direction = match flow.next() {
                    Some(dir) => dir,
                    None => {
                        flow = input.chars();
                        flow.next().unwrap()
                    }
                };
                // _draw_chamber_with_stone(&chamber, &stone);
                match jet_direction {
                    '<' => stone.move_sideways(-1, &chamber), //Move left,
                    '>' => stone.move_sideways(1, &chamber), // Move right,
                    _ => false,
                };
                // _draw_chamber_with_stone(&chamber, &stone);

                // Stone fallig one down
                if !stone.fall_down(&chamber) {
                    // Add stone to chamber
                    for y in 0..stone.shape.len() {
                        for x in 0..stone.shape[y].len() {
                            if stone.shape[y][x] == '#' {
                                chamber[stone.y + y][stone.x + x] = '#'
                            }
                        }
                    }
                    
                    let highest_stone_point = stone.y + stone.height;
                    if highest_point < highest_stone_point {
                        highest_point = highest_stone_point;
                        for _i in 0..max(0, highest_point + 4 - chamber.len()) {
                            chamber.push(new_line.clone());
                        }
                    }
                    break;
                }
            }
        }

        let result = highest_point;
        // _draw_chamber(&chamber);
        println!("Day 17 - Part 1: Result is {}", result);
    }
}

fn _draw_chamber(chamber: &Vec<Vec<char>>) {
    let mut rev_chamber = chamber.clone();
    rev_chamber.reverse();
    for y in 0..20 {
        print!("|");
        for x in &rev_chamber[y] {
            print!("{}", x)
        }
        print!("|\n");
    }
    println!("+-------+\n\n");
}


fn _draw_chamber_with_stone(chamber: &Vec<Vec<char>>, stone: &Stone) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut rev_chamber = chamber.clone();
    for y in 0..stone.shape.len() {
        for x in 0..stone.shape[y].len() {
            if stone.shape[y][x] == '#' && stone.y + y < rev_chamber.len() && stone.x + x < rev_chamber[0].len() {
                rev_chamber[stone.y + y][stone.x + x] = '@'
            }
        }
    }
    rev_chamber.reverse();
    for y in rev_chamber {
        print!("|");
        for x in y {
            print!("{}", x)
        }
        print!("|\n");
    }
    println!("+-------+\n");
}