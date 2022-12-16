use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 16;

struct Valve {
    name: String,
    flow_rate: u64,
    tunnels_to: Vec<String>,
}
impl Valve {
    fn new_from_str(input: &str) -> Self {
        let mut input_split = input.split("; ");

        let valve_input = input_split.next().unwrap();
        let valve_regex = Regex::new(r"Valve (\w+) has flow rate=(\d+)").unwrap();
        let valve_captures = valve_regex.captures(valve_input).unwrap();
        let name = valve_captures.get(1).unwrap().as_str().to_owned();
        let flow_rate = str::parse::<u64>(valve_captures.get(2).unwrap().as_str()).unwrap();

        let tunnel_input = input_split.next().unwrap();
        let tunnel_input_stripped: &str;
        if tunnel_input.starts_with("tunnel leads to valve ") {
            tunnel_input_stripped = tunnel_input.strip_prefix("tunnel leads to valve ").unwrap();
        } else {
            tunnel_input_stripped = tunnel_input
                .strip_prefix("tunnels lead to valves ")
                .unwrap();
        }
        let tunnels_to: Vec<String> = tunnel_input_stripped
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        Self {
            name,
            flow_rate,
            tunnels_to,
        }
    }
}

fn input() -> (HashMap<String, Valve>, String) {
    let raw = input_raw(DAY);
    let lines: Vec<&str> = raw.split("\n").collect();
    let mut result = HashMap::with_capacity(lines.len());
    let mut first_name: Option<String> = None;
    for line in lines {
        let new_valve = Valve::new_from_str(line);
        if first_name.is_none() {
            first_name = Some(new_valve.name.clone());
        }
        result.insert(new_valve.name.clone(), new_valve);
    }

    (result, first_name.unwrap())
}

#[derive(Debug)]
enum Choice {
    Open(String),
    MoveTo(String),
}

#[derive(Clone)]
struct ChoiceNode {
    valves_open: HashSet<String>,
    choices: Vec<Choice>,
    flow_rate: u64,
    total_pressure: u64,
}
impl ChoiceNode {
    fn new(
        valves_open: HashSet<String>,
        valve: &Valve,
        flow_rate: u64,
        total_pressure: u64,
    ) -> Self {
        let mut choices: Vec<Choice> = Vec::new();
        let current_location = valve.name.clone();
        if !valves_open.contains(&current_location) {
            if valve.flow_rate > 0 {
                choices.push(Choice::Open(current_location.clone()));
            }
        }
        for tunnel in valve.tunnels_to.iter() {
            choices.push(Choice::MoveTo(tunnel.clone()));
        }
        let total_pressure = total_pressure + flow_rate;

        Self {
            valves_open,
            choices,
            flow_rate,
            total_pressure,
        }
    }
    // fn get_total_flow(&self, valve_list: &HashMap<String, Valve>) -> u64 {
    //     let mut total = 0u64;
    //     for open_valve in &self.valves_open {
    //         let valve = valve_list.get(open_valve).unwrap();
    //         total += valve.flow_rate;
    //     }

    //     total
    // }
}

#[derive(Clone)]
struct ChoiceTree {
    depth: usize,
    node: ChoiceNode,
}
impl ChoiceTree {
    fn new(depth: usize, node: ChoiceNode) -> Self {
        Self { depth, node }
    }
}

fn find_the_most_pressure(
    max_depth: usize,
    tree: &mut ChoiceTree,
    valve_list: &HashMap<String, Valve>,
) -> Vec<ChoiceTree> {
    // println!("{} DEPTH", tree.depth);
    if tree.depth == max_depth {
        // if tree.node.total_pressure > 0 {
        //     println!("RETURNING: {}", tree.node.total_pressure);
        // }
        return vec![tree.clone()];
    }
    let choices = &tree.node.choices;
    // let mut highest_pressure_seen = 0u64;
    // let mut highest_pressure_tree: Option<ChoiceTree> = None;
    let mut result: Vec<ChoiceTree> = Vec::new();
    for choice in choices {
        match choice {
            Choice::Open(open_me) => {
                let current_valve = valve_list.get(open_me).unwrap();
                let mut new_valves_open = tree.node.valves_open.clone();
                new_valves_open.insert(open_me.clone());
                let flow_rate = tree.node.flow_rate + current_valve.flow_rate;
                let new_node = ChoiceNode::new(
                    new_valves_open,
                    current_valve,
                    flow_rate,
                    tree.node.total_pressure,
                );
                let mut new_tree_node = ChoiceTree::new(tree.depth + 1, new_node);
                let found_trees = find_the_most_pressure(max_depth, &mut new_tree_node, valve_list);
                for found_tree in found_trees {}
                // match best_found {
                //     Some(best) => if best.node.total_pressure > highest_pressure_seen {},
                // }
                // if best_found > highest_pressure_seen {
                //     highest_pressure_seen = new_pressure;
                // }
            }
            Choice::MoveTo(move_to_me) => {
                let move_to_valve = valve_list.get(move_to_me).unwrap();
                let flow_rate = tree.node.flow_rate;
                let new_node = ChoiceNode::new(
                    tree.node.valves_open.clone(),
                    move_to_valve,
                    flow_rate,
                    tree.node.total_pressure,
                );
                let mut new_tree_node = ChoiceTree::new(tree.depth + 1, new_node);
                let new_pressure =
                    find_the_most_pressure(max_depth, &mut new_tree_node, valve_list);
                if new_pressure > highest_pressure_seen {
                    highest_pressure_seen = new_pressure;
                }
            }
        }
    }

    highest_pressure_seen
}

pub fn d16s1(submit: bool) {
    let (input, first_name) = input();
    let first_valve = input.get(&first_name).unwrap();
    let root_node = ChoiceNode::new(HashSet::new(), first_valve, 0, 0);
    let mut tree_root = ChoiceTree::new(0usize, root_node);
    let answer = find_the_most_pressure(30, &mut tree_root, &input);
    final_answer(answer, submit, DAY, 1);
}

pub fn d16s2(_submit: bool) {}
