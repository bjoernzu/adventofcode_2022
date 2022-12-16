use std::{
    borrow::Borrow,
    cmp::{max, min},
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::read_input;

struct Node {
    name: String,
    valve_open: bool,
    flow: u32,
    neighbors: Vec<String>,
}

impl Node {
    fn new<'a>(name: &'a str, flow: u32, neighbors: Vec<String>) -> Node {
        Node {
            name: name.to_string(),
            valve_open: false,
            flow,
            neighbors,
        }
    }

    fn calculate_flow(
        &self,
        min_left: u32,
        closed_valves: &String,
        node_list: &HashMap<&str, Node>,
        node_value_at_min: &mut HashMap<(String, u32, String), u32>,
    ) -> (String, u32) {
        let mut value_open_valve = 0;
        if min_left == 0 {
            return ("".to_string(), 0);
        }
        // Calculate value of opening own valve
        if closed_valves.contains(self.name.borrow() as &str) {
            let mut new_closed_valves = closed_valves.clone();
            new_closed_valves = new_closed_valves.replace(&self.name, "");
            let value =
                self.calculate_flow(min_left - 1, &new_closed_valves, node_list, node_value_at_min);
            value_open_valve = min_left * self.flow + value.1;
        }

        let mut best_action = self.name.clone();
        let mut flow_best_action = value_open_valve;

        for n in &self.neighbors {
            let mut value = ("".to_string(), 0);
            if node_value_at_min.contains_key(&(n.to_string(), min_left, closed_valves.clone())) {
                value = (
                    n.to_string(),
                    *node_value_at_min.get(&(n.to_string(), min_left, closed_valves.clone())).unwrap(),
                );
            } else {
                let node = &node_list[n.borrow() as &str];
                value =
                    node.calculate_flow(min_left - 1, closed_valves, node_list, node_value_at_min);
                node_value_at_min.insert((n.to_string(), min_left, closed_valves.clone()), value.1);
            }
            if value.1 > flow_best_action {
                flow_best_action = value.1;
                best_action = value.0;
            }
        }
        // println!(
        //     "Best Action with {} min left at {} is {} with a flow of: {}, Valves closed: {}",
        //     min_left, self.name, best_action, flow_best_action, closed_valves
        // );
        return (self.name.clone(), flow_best_action);
    }
}

pub struct Day161;
impl Day161 {
    pub fn run(&self) {
        let filename = "input/day161.txt";
        let input = read_input(filename);

        let mut nodes = HashMap::new();
        let mut node_value_at_min: HashMap<(String, u32, String), u32> = HashMap::new();

        let mut closed_valves = "".to_string();


        for line in input.lines() {
            // Parse the input. Ugly but works
            let parts: Vec<&str> = line.split("=").collect();
            let name_part: Vec<&str> = parts[0].split(" ").collect();
            let name = name_part[1];
            let second_parts: Vec<&str>;
            if parts[1].contains("tunnels") {
                second_parts = parts[1].split("; tunnels lead to valves ").collect();
            } else {
                second_parts = parts[1].split("; tunnel leads to valve ").collect();
            }
            println!(
                "name: {}, flow: {}, neighbors {}",
                name, second_parts[0], second_parts[1]
            );
            let flow: u32 = second_parts[0].parse().unwrap();
            let neighbors: Vec<String> =
            second_parts[1].split(", ").map(|s| s.to_string()).collect();
            // Don't try to open valves with no flow
            if flow > 0 {
                closed_valves = closed_valves + name;
            }
            nodes.insert(name, Node::new(name, flow, neighbors));
        }

        let minutes: u32 = 29;
        let starting_node = "AA";
        let current_node: &Node = &nodes[starting_node];
        let result =
            current_node.calculate_flow(minutes, &closed_valves.to_string(), &nodes, &mut node_value_at_min);
        println!("Day 16 - Part 1: Result is {}", result.1);
    }
}
