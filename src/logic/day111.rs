use crate::read_input;

use std::cell::{RefCell};
use std::rc::Rc;

struct Monkey {
    items: Rc<RefCell<Vec<u64>>>,
    operation_rule: String,
    test_number: u64,
    target_true: usize,
    target_false: usize,
    inspections: i32,
}

impl Monkey {
    fn new(
        starting_items: Vec<u64>,
        operation_rule: String,
        test_number: u64,
        target_true: usize,
        target_false: usize,
    ) -> Monkey {
        Monkey {
            items: Rc::new(RefCell::new(starting_items)),
            operation_rule,
            test_number,
            target_true,
            target_false,
            inspections: 0,
        }
    }

    fn inspect(&mut self) -> Vec<(usize, u64)> {
        let num_items = self.items.borrow_mut().len();
        let mut result = Vec::new();
        for _i in 0..num_items {
            let mut item = self.items.borrow_mut().remove(0);
            item = self.inspect_item(item);
            self.inspections += 1;
            result.push(self.test_item_and_aim_throw(item));
        }

        return result;
    }

    fn inspect_item(&self, item: u64) -> u64 {
        let old_worry_level = item;
        
        // Do the inspection 
        let parts: Vec<&str> = self.operation_rule.split_whitespace().collect();
        let left_value = match parts[0] {
            "old" => old_worry_level,
            _ => parts[0].parse::<u64>().unwrap(),
        };

        let right_value = match parts[2] {
            "old" => old_worry_level,
            _ => parts[2].parse::<u64>().unwrap(),
        };

        let new_worry_level = match parts[1] {
            "*" => left_value * right_value,
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            _ => 0
        };

        // Reduce worry level
        return new_worry_level / 3 as u64;
    }

    fn test_item_and_aim_throw(&mut self, item: u64) -> (usize, u64) {
        let mut target_monkey = self.target_false;
        if item % self.test_number == 0 {
            target_monkey = self.target_true;
        }
        
        return (target_monkey, item);
        // let mut target_monkey_ref: RefMut<'_, Vec<Monkey>> = self.monkey_list.borrow_mut(); 
        // target_monkey_ref[target_monkey].catch_item(item);
    }

    fn catch_item(&mut self, item: u64) {
        self.items.borrow_mut().push(item);
    }

    // fn print_items(&self) {
    //     for i in self.items.borrow_mut().iter() {
    //         print!("{} ,", i)
    //     }
    //     print!("\n");
    // }
}

pub struct Day111;
impl Day111 {
    pub fn run(&self) {
        let filename = "input/day111.txt";
        let input = read_input(filename);

        let monkey_list: Vec<Monkey> = Vec::new();
        let monkey_list_ref = Rc::new(RefCell::new(monkey_list));
        let mut num_monkeys = 0;
        let mut lines = input.lines();

        loop {
            let mut line = match lines.next() {
                Some(line) => line,
                None => break,
            };
            if line.starts_with("Monkey") {
                num_monkeys += 1;
                // Get the starting items
                line = lines.next().unwrap();
                let starting_items: Vec<u64> = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|i| i.parse().unwrap())
                    .collect();

                // Get the operation rule
                let operation_rule: String = lines
                    .next()
                    .unwrap()
                    .split("= ")
                    .nth(1)
                    .unwrap()
                    .to_string();

                // Get the Test number
                let test_number: u64 = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                // Get the monkey to throw the item to if the test is successfull
                let target_true = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                // Get the monkey to throw the item to if the test is successfull
                let target_false = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                let monkey = Monkey::new(
                    starting_items,
                    operation_rule,
                    test_number,
                    target_true,
                    target_false,
                );

                monkey_list_ref.borrow_mut().push(monkey);
                if lines.next().is_none() {
                    break;
                }
            }
        };

        for _round in 0..20 {
            // println!("Round {}", round);
            for m in 0..num_monkeys {
                let result = monkey_list_ref.borrow_mut()[m].inspect();
                for throw in result {
                    monkey_list_ref.borrow_mut()[throw.0].catch_item(throw.1);
                }
            }
            // for m in 0..num_monkeys {
            //     print!("Monkey {} items: ", m);
            //     monkey_list_ref.borrow_mut()[m].print_items();
            // }
        }

        let mut inspection_list = Vec::new();
        for m in 0..num_monkeys {
            inspection_list.push(monkey_list_ref.borrow_mut()[m].inspections)
        }
        inspection_list.sort();
        inspection_list.reverse();

        let result = inspection_list[0] * inspection_list[1];


        // Print the result
        println!("Day 11 - Part 1: Result is {}", result);
    }
}