use std::env;

mod logic;
mod utils;
use std::collections::HashMap;
use utils::read_input;

fn main() {
    let mut puzzles: HashMap<(u8, u8), Box<dyn logic::Puzzle>> = HashMap::new();

    puzzles.insert((1, 1), Box::new(logic::Day011));
    puzzles.insert((1, 2), Box::new(logic::Day012));
    puzzles.insert((2, 1), Box::new(logic::Day021));
    puzzles.insert((2, 2), Box::new(logic::Day022));
    puzzles.insert((3, 1), Box::new(logic::Day031));
    puzzles.insert((3, 2), Box::new(logic::Day032));
    puzzles.insert((4, 1), Box::new(logic::Day041));
    puzzles.insert((4, 2), Box::new(logic::Day042));
    puzzles.insert((5, 1), Box::new(logic::Day051));
    puzzles.insert((5, 2), Box::new(logic::Day052));
    puzzles.insert((6, 1), Box::new(logic::Day061));
    puzzles.insert((6, 2), Box::new(logic::Day062));
    // puzzles.insert((7,1), Box::new(logic::Day071));
    // puzzles.insert((7,2), Box::new(logic::Day072));
    puzzles.insert((8, 1), Box::new(logic::Day081));
    puzzles.insert((8, 2), Box::new(logic::Day082));
    puzzles.insert((9, 1), Box::new(logic::Day091));
    puzzles.insert((9, 2), Box::new(logic::Day092));
    puzzles.insert((10, 1), Box::new(logic::Day101));
    puzzles.insert((10, 2), Box::new(logic::Day102));
    puzzles.insert((11, 1), Box::new(logic::Day111));
    puzzles.insert((11, 2), Box::new(logic::Day112));
    puzzles.insert((12, 1), Box::new(logic::Day121));
    puzzles.insert((12, 2), Box::new(logic::Day122));
    puzzles.insert((13, 1), Box::new(logic::Day131));
    puzzles.insert((13, 2), Box::new(logic::Day132));
    puzzles.insert((14, 1), Box::new(logic::Day141));
    puzzles.insert((14, 2), Box::new(logic::Day142));
    puzzles.insert((15, 1), Box::new(logic::Day151));
    puzzles.insert((15, 2), Box::new(logic::Day152));
    puzzles.insert((16, 1), Box::new(logic::Day161));
    puzzles.insert((16, 2), Box::new(logic::Day162));
    puzzles.insert((17, 1), Box::new(logic::Day171));
    puzzles.insert((17, 2), Box::new(logic::Day172));
    puzzles.insert((18, 1), Box::new(logic::Day181));
    // puzzles.insert((18, 2), Box::new(logic::Day182));

    // Check what to execute
    // Default: Only execute last puzzle
    // "all": Execute all puzzles
    // "16": Execute all puzzles for day 16
    // "16 1": Execute only part 1 for day 16
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => puzzles.iter().last().unwrap().1.run(),
        2 => match args[1].as_str() {
            "all" => for p in puzzles.iter() {p.1.run()},
            _ => {
                let day = args[1].parse::<u8>().unwrap();
                solve_puzzle(puzzles.get(&(day, 1 as u8)));
                solve_puzzle(puzzles.get(&(day, 2 as u8)))
            },
        },
        3 => solve_puzzle(puzzles.get(&(
            args[1].parse::<u8>().unwrap(),
            args[2].parse::<u8>().unwrap(),
        ))),
        _ => {}
    }
}

fn solve_puzzle(p: Option<&Box<dyn logic::Puzzle>>) {
    match p {
        Some(puzzle) => puzzle.run(),
        _ => println!(), 
    }
}