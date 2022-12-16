use std::{
    borrow::Borrow,
    collections::{HashMap},
};

use crate::read_input;

#[derive(Clone)]
struct Node {
    name: String,
    flow: u32,
    neighbors: Vec<String>,
}

impl Node {
    fn new<'a>(name: &'a str, flow: u32, neighbors: Vec<String>) -> Node {
        Node {
            name: name.to_string(),
            flow,
            neighbors,
        }
    }

    fn calculate_flow(
        &self,
        min_left: u32,
        closed_valves: &String,
        node_list: &HashMap<&str, Node>,
        node_value_at_min: &mut HashMap<(String, u32, String), (String, u32)>,
    ) -> (String, u32) {
        let mut result_closed_valves = closed_valves.clone();
        let mut value_open_valve = 0;
        if min_left == 0 {
            return (result_closed_valves, 0);
        }
        // Calculate value of opening own valve
        if closed_valves.contains(self.name.borrow() as &str) {
            let new_closed_valves = result_closed_valves.replace(&self.name, "");
            let value =
                self.calculate_flow(min_left - 1, &new_closed_valves, node_list, node_value_at_min);
            value_open_valve = min_left * self.flow + value.1;
            result_closed_valves = value.0;
        }

        let mut flow_best_action = value_open_valve;

        for n in &self.neighbors {
            let value: (String, u32);
            if node_value_at_min.contains_key(&(n.to_string(), min_left, closed_valves.clone())) {
                value = node_value_at_min.get(&(n.to_string(), min_left, closed_valves.clone())).unwrap().clone();
            } else {
                let node = &node_list[n.borrow() as &str];
                value =
                    node.calculate_flow(min_left - 1, closed_valves, node_list, node_value_at_min);
                node_value_at_min.insert((n.to_string(), min_left, closed_valves.clone()), value.clone());
            }
            if value.1 > flow_best_action {
                flow_best_action = value.1;
                result_closed_valves = value.0;
            }
        }
        return (result_closed_valves, flow_best_action);
    }
}

pub struct Day162;
impl Day162 {
    pub fn run(&self) {
        let filename = "input/day162.txt";
        let input = read_input(filename);

        let mut nodes = HashMap::new();
        let mut node_value_at_min: HashMap<(String, u32, String), (String, u32)> = HashMap::new();

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
            let flow: u32 = second_parts[0].parse().unwrap();
            let neighbors: Vec<String> =
            second_parts[1].split(", ").map(|s| s.to_string()).collect();
            // Don't try to open valves with no flow
            if flow > 0 {
                closed_valves = closed_valves + name;
            }
            nodes.insert(name, Node::new(name, flow, neighbors));
        }

        let minutes: u32 = 25;
        let starting_node = "AA";
        let current_node: &Node = &nodes[starting_node];
        let result =
            current_node.calculate_flow(minutes, &closed_valves.to_string(), &nodes, &mut node_value_at_min);

        let ele_node: &Node = &nodes[starting_node];
        node_value_at_min = HashMap::new();
        let eleresult = ele_node.calculate_flow(minutes, &result.0, &nodes, &mut node_value_at_min);
        
        println!("Day 16 - Part 2: Result is {}", result.1 + eleresult.1);
    }
}
