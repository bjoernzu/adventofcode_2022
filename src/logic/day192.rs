use std::cmp::max;
use std::collections::HashMap;

use regex::Regex;

use crate::logic::Puzzle;
use crate::read_input;

type Materials = [u8; 4];
type State = (u8, Materials, Materials);
type Result = HashMap<State, u32>;

const NAMES: [&str; 4] = ["Ore", "Clay", "Obsidian", "Geode"];

struct Blueprint {
    id: u8,
    costs: [Materials; 4],
    max_costs: Materials,
}

impl Blueprint {
    fn new(
        id: u8,
        cost_ore: Materials,
        cost_clay: Materials,
        cost_obsidian: Materials,
        cost_geode: Materials,
    ) -> Blueprint {
        let mut max_costs = [0;4];
        for i in 0..3 {
            max_costs[i] = max(cost_ore[i], max(cost_clay[i], max(cost_obsidian[i], cost_geode[i])))
        }
        max_costs[3] = 99;
        Blueprint {
            id,
            costs: [cost_ore, cost_clay, cost_obsidian, cost_geode],
            max_costs
        }
    }

    fn optimize_path(&self, state: &State, closed_list: &mut Result) -> u32 {
        if closed_list.contains_key(state) {
            return closed_list.get(state).unwrap().clone();
        }

        // print!("Checking state: ");
        // print_state(&state);
        if state.0 == 1 {
            return state.1[3] as u32 + state.2[3] as u32;
        }

        let mut new_states: Vec<State> = Vec::new();

        let mut gen_state = state.clone();
        
        // Check options to build robots
        let mut build_geode_robot = false;
        for r in (0..=3).rev() {
            // println!("Checking to build {} robot", NAMES[r]);
            if state.1[r] < self.max_costs[r] // Don't produce more robots than we can spend in max 1 round
            && state.2[r] as u32 <= state.0 as u32 * self.max_costs[r] as u32 //Don't produce robot if we cannot spend it later on
            && state.2[0] >= self.costs[r][0]
            && state.2[1] >= self.costs[r][1]
            && state.2[2] >= self.costs[r][2]
            && state.2[3] >= self.costs[r][3]
            {
                let mut new_state = state.clone();
                let mut new_robots = [0; 4];
                new_robots[r] = 1;
                new_state.0 = state.0 - 1;
                new_state.1 = add_materials(&state.1, &new_robots); // Add the new robot to be build
                new_state.2 = substract_materials(&state.2, &self.costs[r]); // remove the material cost
                new_state.2 = add_materials(&new_state.2, &state.1); // add the production of the original robots
                // print!("Adding new path: ");
                // print_state(&new_state);
                new_states.push(new_state);
                if r == 3 {
                    build_geode_robot = true;
                    break; // Assume that building a Geode Collector if possibe is always the best option
                }
                if r == 2 {
                    break; // Asuume that an Obsidian collector is still better than clay or ore
                }
            }
        }
        // Add generic state without building anything
        if !build_geode_robot {
            gen_state.0 = state.0 - 1;
            gen_state.2 = add_materials(&state.2, &state.1);
            new_states.push(gen_state);
        }
            
        let mut best_result: u32 = 0;
        for s in new_states {
            let result = self.optimize_path(&s, closed_list);
            closed_list.insert(s, result);
            best_result = max(best_result, result);
        }

        return best_result;
    }
}

pub struct Day192;
impl Puzzle for Day192 {
    fn run(&self) {
        let filename = "input/day192.txt";
        let input = read_input(filename);

        let mut blueprints: Vec<Blueprint> = Vec::new();

        let re = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

        for cap in re.captures_iter(input.as_str()) {
            blueprints.push(Blueprint::new(
                cap[1].parse::<u8>().unwrap(),
                [cap[2].parse::<u8>().unwrap(), 0, 0, 0],
                [cap[3].parse::<u8>().unwrap(), 0, 0, 0],
                [
                    cap[4].parse::<u8>().unwrap(),
                    cap[5].parse::<u8>().unwrap(),
                    0,
                    0,
                ],
                [
                    cap[6].parse::<u8>().unwrap(),
                    0,
                    cap[7].parse::<u8>().unwrap(),
                    0,
                ],
            ));
        }

        let time = 32;
        let state: State = (time, [1, 0, 0, 0], [0, 0, 0, 0]);

        let mut result = 1;
        for b in blueprints {    
            let mut closed_list = HashMap::new();
            let r = b.optimize_path(&state, &mut closed_list);
            result = r * result;
            println!("Blueprint {}: {}", b.id, r);
        }

            

        println!("Day 19 - Part 2: Result is {}", result);
    }
}

fn add_materials(m1: &Materials, m2: &Materials) -> Materials {
    let mut result: Materials = [0; 4];
    for i in 0..4 {
        result[i] = m1[i] + m2[i]
    }
    return result;
}

fn substract_materials(m1: &Materials, m2: &Materials) -> Materials {
    let mut result: Materials = [0; 4];
    for i in 0..4 {
        result[i] = m1[i] - m2[i]
    }
    return result;
}

fn print_state(state: &State) {
    println!(
        "Minute {}, robots: {}, {}, {}, {}; materials {}, {}, {}, {}",
        state.0,
        state.1[0],
        state.1[1],
        state.1[2],
        state.1[3],
        state.2[0],
        state.2[1],
        state.2[2],
        state.2[3]
    );
}