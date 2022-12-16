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
    None,
}

struct ChoiceNode {
    valves_open: HashSet<String>,
    current_location: String,
    choices: Vec<Choice>,
    total_pressure: u64,
}
impl ChoiceNode {
    fn new(valves_open: HashSet<String>, valve: &Valve, total_pressure: u64) -> Self {
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
        // You may always choose to do nothing.
        choices.push(Choice::None);

        Self {
            valves_open,
            current_location,
            choices,
            total_pressure,
        }
    }
    fn get_total_flow(&self, valve_list: &HashMap<String, Valve>) -> u64 {
        let mut total = 0u64;
        for open_valve in &self.valves_open {
            let valve = valve_list.get(open_valve).unwrap();
            total += valve.flow_rate;
        }

        total
    }
}

struct ChoiceTree {
    depth: usize,
    node: ChoiceNode,
    // branches: Vec<ChoiceNode>,
}
impl ChoiceTree {
    fn new(depth: usize, node: ChoiceNode) -> Self {
        Self {
            depth,
            node,
            // branches,
        }
    }
}

fn find_the_most_pressure(
    max_depth: usize,
    tree: &mut ChoiceTree,
    valve_list: &HashMap<String, Valve>,
) -> u64 {
    // println!("{} DEPTH", tree.depth);
    if tree.depth == max_depth {
        // if tree.node.total_pressure > 0 {
        //     println!("RETURNING: {}", tree.node.total_pressure);
        // }
        return tree.node.total_pressure;
    }
    let choices = &tree.node.choices;
    let mut highest_pressure_seen = 0u64;
    for choice in choices {
        match choice {
            Choice::Open(open_me) => {
                let current_valve = valve_list.get(open_me).unwrap();
                let mut new_valves_open = tree.node.valves_open.clone();
                let flow_rate = tree.node.get_total_flow(valve_list);
                // TODO: the flow rate needs incremented by the new valve
                new_valves_open.insert(open_me.clone());
                let new_node = ChoiceNode::new(
                    new_valves_open,
                    current_valve,
                    tree.node.total_pressure + flow_rate,
                );
                let mut new_tree_node = ChoiceTree::new(tree.depth + 1, new_node);
                let new_pressure =
                    find_the_most_pressure(max_depth, &mut new_tree_node, valve_list);
                if new_pressure > highest_pressure_seen {
                    highest_pressure_seen = new_pressure;
                }
            }
            Choice::MoveTo(move_to_me) => {
                let move_to_valve = valve_list.get(move_to_me).unwrap();
                let flow_rate = tree.node.get_total_flow(valve_list);
                let new_node = ChoiceNode::new(
                    tree.node.valves_open.clone(),
                    move_to_valve,
                    tree.node.total_pressure + flow_rate,
                );
                let mut new_tree_node = ChoiceTree::new(tree.depth + 1, new_node);
                let new_pressure =
                    find_the_most_pressure(max_depth, &mut new_tree_node, valve_list);
                if new_pressure > highest_pressure_seen {
                    highest_pressure_seen = new_pressure;
                }
            }
            Choice::None => {
                // let move_to_valve = valve_list.get(move_to_me).unwrap();
                // let flow_rate = tree.node.get_total_flow(valve_list);
                // let new_node = ChoiceNode::new(
                //     tree.node.valves_open.clone(),
                //     move_to_valve,
                //     tree.node.total_pressure + flow_rate,
                // );
                // let mut new_tree_node = ChoiceTree::new(tree.depth + 1, new_node);
                // let new_pressure =
                //     find_the_most_pressure(max_depth, &mut new_tree_node, valve_list);
                // if new_pressure > highest_pressure_seen {
                //     highest_pressure_seen = new_pressure;
                // }

                // This is a strange consideration,
                // but if we ever get here,
                // a MoveTo will accomplish the same thing...
                // return 0;
                continue;
            }
        }
    }

    highest_pressure_seen
}

pub fn d16s1(submit: bool) {
    let (input, first_name) = input();
    let first_valve = input.get(&first_name).unwrap();
    let root_node = ChoiceNode::new(HashSet::new(), first_valve, 0);
    let mut tree_root = ChoiceTree::new(0usize, root_node);
    let answer = find_the_most_pressure(30, &mut tree_root, &input);
    // for minute in 1..=30 {
    //     let leaves = tree_root
    //     for depth in 0..minute {

    //     }
    // }
    // let valves_open =
    final_answer(answer, submit, DAY, 1);
}

pub fn d16s2(_submit: bool) {
    // let (input, first_name) = input();
    // final_answer(input.len(), submit, DAY, 2);
}
