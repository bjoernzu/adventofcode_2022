use crate::read_input;
use crate::logic::Puzzle;use std::cmp::min;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Packet {
    Packetlist(Vec<Packet>),
    Value(i32)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // If both sides are values, return the comparison of these values
            (Packet::Value(s), Packet::Value(o)) => s.cmp(o),
            
            // If only one side is a value, make it a Packetlist and return the comparison of two packet lists
            (Packet::Value(s), Packet::Packetlist(_)) => Packet::Packetlist(vec![Packet::Value(*s)]).cmp(other),
            (Packet::Packetlist(_), Packet::Value(o)) => self.cmp(&Packet::Packetlist(vec![Packet::Value(*o)])),

            // If both sides are Packetlists, loop through the elements until a clear comparison is found
            (Packet::Packetlist(s), Packet::Packetlist(o)) => {
                // Loop through the elements and use comparison we defined above
                for i in 0..min(s.len(), o.len()) {
                    match s[i].cmp(&o[i]) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        _ => {}
                    }
                }

                // If loop does not come to a clear result, need to check which list is longer.
                if s.len() < o.len() {
                    return Ordering::Less
                }
                else if s.len() > o.len() {
                    return Ordering::Greater
                }
                else {
                    return Ordering::Equal
                }

            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


pub struct Day132;
impl Puzzle for Day132 {
    fn run(&self) {
        let filename = "input/day132.txt";
        let input = read_input(filename);

        let mut packets = Vec::new();
        packets.push(parse_line("[[2]]"));
        packets.push(parse_line("[[6]]"));
        
        for line in input.lines() {
            if !line.is_empty() {
                packets.push(parse_line(line))
            }
        }

        packets.sort();
        let pos_1 = packets.iter().position(|p| p == &parse_line("[[2]]")).unwrap() + 1;
        let pos_2 = packets.iter().position(|p| p == &parse_line("[[6]]")).unwrap() + 1;
        // println!("[[2]] Position: {}, [[6]] Position: {}", pos_1, pos_2);
        // Print the result
        println!("Day 13 - Part 2: Result is {}", pos_1 * pos_2);
    }
}




fn parse_line(line: &str) -> Packet {
    // Add ","" to make sure we can split the line and get the right elements
    let variable_to_bind_elements = line.replace("[", "[,").replace("]", ",]");
    let mut elements = variable_to_bind_elements.split(",").collect();

    return parse_elements(&mut elements).pop().unwrap();

}

fn parse_elements(elements: &mut Vec<&str>) -> Vec<Packet> {
    let mut packetlist: Vec<Packet> = Vec::new();

    loop {
        if elements.len() == 0 {
            break;
        }
        let e = elements.remove(0);
        match e {
            "[" => { packetlist.push(Packet::Packetlist(parse_elements(elements)))}, 
            "]" => { return packetlist},
            "" => {},
            _ => packetlist.push(Packet::Value(e.parse().unwrap()))
        }

    }

    return packetlist;
}

fn _get_string(packet: &Packet) -> String {
    match packet {
        Packet::Value(p) => return p.to_string(),
        Packet::Packetlist(pl) => {
            let strings: Vec<String> = pl.iter().map(|p| _get_string(p)).collect();
            return format!("[{}]", strings.join(",") );
        }
    };
}
