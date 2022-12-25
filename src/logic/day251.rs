use crate::logic::Puzzle;
use crate::read_input;

const DEBUG: bool = false;

fn snafu_to_dec(line: &str) -> i64 {
    let mut dec = 0;
    let mut exp = 0;
    for c in line.chars().rev() {
        match c {
            '=' => dec += -2 * 5_i64.pow(exp),
            '-' => dec += -1 * 5_i64.pow(exp),
            '0' => dec += 0 * 5_i64.pow(exp),
            '1' => dec += 1 * 5_i64.pow(exp),
            '2' => dec += 2 * 5_i64.pow(exp),
            _ => {}
        }
        exp += 1;
    }
    return dec;
}

fn dec_to_snafu(dec: i64) -> String {
    let mut snafu: String = "".to_string();
    let mut number = dec;
    while number > 0 {
        let position = number % 5;
        number = number / 5;
        let print_position: String;

        match position {
            4 => { print_position = format!("-"); number += 1;},
            3 => { print_position = format!("="); number += 1;}
            _ => {print_position =format!("{}", position);}
        }


        snafu = format!("{}{}", print_position, snafu);

    }

    return snafu;
}

pub struct Day251;
impl Puzzle for Day251 {
    fn run(&self) {
        let filename = "input/day251.txt";
        let input = read_input(filename);

        let mut sum: i64 = 0;

        // Parse the input
        for line in input.lines() {
            if !line.is_empty() {
                let dec = snafu_to_dec(line);
                if DEBUG {
                    println!("SNAFU number {} is {} in dec", line, dec);
                }
                sum += dec;
            }
        }

        let result = dec_to_snafu(sum);

        println!("Day 24 - Part 2: Result is {} (dec: {})", result, sum);
    }
}