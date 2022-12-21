use std::collections::HashMap;

use crate::logic::Puzzle;
use crate::read_input;

enum Monkey {
    Value(ValueMonkey),
    Calculation(CalculationMonkey),
}

struct ValueMonkey {
    id: String,
    value: i64,
}

struct CalculationMonkey {
    left: String,
    right: String,
    operand: char,
}

trait Calculation {
    fn calculate(&self, monkeys: &HashMap<String, Monkey>) -> i64;
    fn calculate_human(&self, monkeys: &HashMap<String, Monkey>, target_value: i64) -> i64;
    fn is_human_path(&self, monkeys: &HashMap<String, Monkey>) -> bool;
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

    fn calculate_human(&self, monkeys: &HashMap<String, Monkey>, target_value: i64) -> i64 {
        match self {
            Monkey::Value(m) => {
                return match m.id.as_str() {
                    "humn" => target_value,
                    _ => m.value,
                }
            }
            Monkey::Calculation(m) => {
                let leftmonkey = monkeys.get(&m.left).unwrap();
                let rightmonkey = monkeys.get(&m.right).unwrap();
                if leftmonkey.is_human_path(monkeys) {
                    let right_result = rightmonkey.calculate(monkeys);
                    return match m.operand {
                        '+' => leftmonkey.calculate_human(monkeys, target_value - right_result), // target: 6, right: 3 => 6 = X + 3 => X = target - right => X = 3
                        '-' => leftmonkey.calculate_human(monkeys, target_value + right_result), // target: 6, right: 3 => 6 = X - 3 => X = target + right => X = 9
                        '*' => leftmonkey.calculate_human(monkeys, target_value / right_result), // target: 6, right: 3 => 6 = X * 3 => X = target / right => X = 2
                        '/' => leftmonkey.calculate_human(monkeys, target_value * right_result), // target: 6, right: 3 => 6 = X / 3 => X = target * right => X = 18
                        '=' => leftmonkey.calculate_human(monkeys, right_result),
                        _ => 0,
                    };
                } else if rightmonkey.is_human_path(monkeys) {
                    let left_result = leftmonkey.calculate(monkeys);
                    return match m.operand {
                        '+' => rightmonkey.calculate_human(monkeys, target_value - left_result), // target: 6, left: 3 => 6 = 3 + X => X = target - left => X = 3
                        '-' => rightmonkey.calculate_human(monkeys, left_result - target_value), // target: 6, left: 3 => 6 = 3 - X => X = left - target => X = -3
                        '*' => rightmonkey.calculate_human(monkeys, target_value / left_result), // target: 6, left: 3 => 6 = 3 * X => X = target / left => X = 2
                        '/' => rightmonkey.calculate_human(monkeys, left_result / target_value), // target: 6, left: 3 => 6 = 3 / X => X = left / target => X = 1/2
                        '=' => rightmonkey.calculate_human(monkeys, left_result),
                        _ => 0,
                    };
                }
                else {
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

    fn is_human_path(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        match self {
            Monkey::Value(m) => return m.id == "humn",
            Monkey::Calculation(m) => {
                let leftmonkey = monkeys.get(&m.left).unwrap();
                let rightmonkey = monkeys.get(&m.right).unwrap();
                return leftmonkey.is_human_path(monkeys) || rightmonkey.is_human_path(monkeys);
            }
        }
    }
}

pub struct Day212;
impl Puzzle for Day212 {
    fn run(&self) {
        let filename = "input/day212.txt";
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
                            id: id.to_string(),
                            value: calculation[0].parse::<i64>().unwrap(),
                        }),
                    );
                } else {
                    let mut operand = calculation[1].chars().nth(0).unwrap();
                    if id == "root" {
                        operand = '=';
                    }
                    monkeys.insert(
                        id.to_string(),
                        Monkey::Calculation(CalculationMonkey {
                            left: calculation[0].to_string(),
                            right: calculation[2].to_string(),
                            operand,
                        }),
                    );
                }
            }
        }

        let root_monkey = monkeys.get("root").unwrap();
        let result = root_monkey.calculate_human(&monkeys, 0);

        println!("Day 21 - Part 2: Result is {}", result);
    }
}
