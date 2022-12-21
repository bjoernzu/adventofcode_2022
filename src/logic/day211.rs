use std::collections::HashMap;

use crate::logic::Puzzle;
use crate::read_input;

enum Monkey {
    Value(ValueMonkey),
    Calculation(CalculationMonkey),
}

struct ValueMonkey {
    value: i64,
}

struct CalculationMonkey {
    left: String,
    right: String,
    operand: char,
}

trait Calculation {
    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> i64;
}

impl Calculation for Monkey {
    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Value(m) => return m.value,
            Monkey::Calculation(m) => {
                let leftmonkey = monkeys.get(&m.left).unwrap();
                let rightmonkey = monkeys.get(&m.right).unwrap();
                return match m.operand {
                    '+' => leftmonkey.calculate(monkeys) + rightmonkey.calculate(monkeys),
                    '-' => leftmonkey.calculate(monkeys) - rightmonkey.calculate(monkeys),
                    '*' => leftmonkey.calculate(monkeys) * rightmonkey.calculate(monkeys),
                    '/' => leftmonkey.calculate(monkeys) / rightmonkey.calculate(monkeys),
                    _ => 0,
                };
            }
        }
    }
}

pub struct Day211;
impl Puzzle for Day211 {
    fn run(&self) {
        let filename = "input/day211.txt";
        let input = read_input(filename);

        let mut monkeys: HashMap<String, Monkey> = HashMap::new();

        for line in input.lines() {
            if !line.is_empty() {
                let parts: Vec<&str> = line.split(": ").collect();
                let id = parts[0];
                let calculation: Vec<&str> = parts[1].split_whitespace().collect();
                if calculation.len() == 1 {
                    monkeys.insert(
                        id.to_string(),
                        Monkey::Value(ValueMonkey {
                            value: calculation[0].parse::<i64>().unwrap(),
                        }),
                    );
                } else {
                    monkeys.insert(
                        id.to_string(),
                        Monkey::Calculation(CalculationMonkey {
                            left: calculation[0].to_string(),
                            right: calculation[2].to_string(),
                            operand: calculation[1].chars().nth(0).unwrap(),
                        }),
                    );
                }
            }
        }

        let root_monkey = monkeys.get("root").unwrap();
        let result = root_monkey.calculate(&monkeys);

        println!("Day 21 - Part 1: Result is {}", result);
    }
}
