use crate::read_input;


pub struct Day102;
impl Day102 {

    
    pub fn run(&self) {
        let filename = "input/day102.txt";
        let input = read_input(filename);

        let mut step: i32 = 1;
        let mut x: i32 = 1;
        
        // Print the result
        println!("Day 10 - Part 2: Result is");

        for line in input.lines() {
            
            // Execute noop
            if line.starts_with("noop") {
                draw(&step, &x);
                step += 1;
            }
            else if line.starts_with("addx") {
                draw(&step, &x);
                step += 1;
                draw(&step, &x);
                step += 1;
                let num = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                x += num;
            }
        }


    }
}

fn draw(step: &i32, x: &i32) {
    let pos = step%40-1;
    if x-1 <= pos && pos <= x+1 {
        print!("#")
    }
    else {
        print!(" ")
    }
    // println!("{} {} {}", step, x, pos);
    if pos + 1 == 0 {
        print!("\n")
    }
    
}