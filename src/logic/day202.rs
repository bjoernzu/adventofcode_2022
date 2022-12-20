use crate::logic::Puzzle;
use crate::read_input;

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Clone)]
struct CryptNumber {
    id: i64,
    value: i64,
}

pub struct Day202;
impl Puzzle for Day202 {
    fn run(&self) {
        let filename = "input/day201.txt";
        let input = read_input(filename);

        let mut input_numbers: Vec<CryptNumber> = Vec::new();
        let mut result_numbers: Vec<CryptNumber>;

        let mut id = 0;
        for line in input.lines() {
            if !line.is_empty() {
                input_numbers.push(CryptNumber{id, value: line.parse::<i64>().unwrap() * DECRYPTION_KEY});
                id += 1;
            }
        }

        result_numbers = input_numbers.clone();
        let num_numbers = result_numbers.len() as i64;

        // Move numbers
        for _i in 0..10 {
            for number in &input_numbers {
                let current_position = result_numbers.iter().position(|n| n.id == number.id).unwrap() as usize;
                let future_position = ((current_position as i64 + number.value % (num_numbers - 1)) + num_numbers - 1) % (num_numbers - 1);
                
                let element = result_numbers.remove(current_position);
                result_numbers.insert(future_position as usize, element);
            }
        }


        // Find zero
        let mut zero_position = 0;
        for i in 0..result_numbers.len() {
            if result_numbers[i].value == 0 {
                zero_position = i;
            }
        }


        let result = result_numbers[(zero_position + 1000) % num_numbers as usize].value +
        result_numbers[(zero_position + 2000) % num_numbers as usize].value +
        result_numbers[(zero_position + 3000) % num_numbers as usize].value;
        println!("Day 20 - Part 2: Result is {}", result);
    }
}
