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

// #[derive(Debug)]
// enum Choice {
//     Open(String),
//     MoveTo(String),
// }

// #[derive(Clone)]
// struct ChoiceNode {
//     valves_open: HashSet<String>,
//     choices: Vec<Choice>,
//     flow_rate: u64,
//     total_pressure: u64,
// }
// impl ChoiceNode {
//     fn new(
//         valves_open: HashSet<String>,
//         valve: &Valve,
//         flow_rate: u64,
//         total_pressure: u64,
//     ) -> Self {
//         let mut choices: Vec<Choice> = Vec::new();
//         let current_location = valve.name.clone();
//         if !valves_open.contains(&current_location) {
//             if valve.flow_rate > 0 {
//                 choices.push(Choice::Open(current_location.clone()));
//             }
//         }
//         for tunnel in valve.tunnels_to.iter() {
//             choices.push(Choice::MoveTo(tunnel.clone()));
//         }
//         let total_pressure = total_pressure + flow_rate;

//         Self {
//             valves_open,
//             choices,
//             flow_rate,
//             total_pressure,
//         }
//     }
// }

// fn get_shortest_path_between_valves(
//     start: &Valve,
//     end: &Valve,
//     map: &HashMap<String, Valve>,
// ) -> Vec<String> {
//     if start.name == end.name {
//         return vec![];
//     }
//     let mut best_path: Option<Vec<String>> = None;
//     for tunnel in start.tunnels_to {
//         if tunnel == end.name {
//             return vec![tunnel];
//         }
//         let try_me = get_shortest_path_between_valves(&map[&tunnel], end, map);
//         match best_path {
//             None => {
//                 best_path = Some(try_me);
//             }
//             Some(best_path_inner) => {
//                 if try_me.len() < best_path_inner.len() {
//                     best_path = Some(try_me);
//                 }
//             }
//         }
//     }
//     return best_path.unwrap();
// }

struct DistanceMap {
    data: HashMap<String, usize>,
}
impl DistanceMap {
    fn key_from_strs(from: &str, to: &str) -> String {
        format!("{}>{}", from, to)
    }
    // fn key_from_strings(from: String, to: String) -> String {
    //     format!("{}>{}", from, to)
    // }
    // fn key_from_valves(from: &Valve, to: &Valve) -> String {
    //     format!("{}>{}", from.name, to.name)
    // }
    fn insert(&mut self, key: String, distance: usize) {
        if self.data.contains_key(&key) {
            return;
        }
        self.data.insert(key, distance);
    }
    fn get_and_unwrap(&self, key: &String) -> usize {
        *self.data.get(key).unwrap()
    }

    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

fn get_shortest_distance(
    start: String,
    end: String,
    valves: &HashMap<String, Valve>,
    visited: &mut HashSet<String>,
) -> Option<usize> {
    let mut depth = 0usize;
    let mut tunnels = vec![start];
    loop {
        let mut check: HashSet<String> = HashSet::new();
        for tunnel in tunnels.iter() {
            if tunnel.as_str() == end.as_str() {
                return Some(depth);
            }
            visited.insert(tunnel.clone());
            for neighbor in valves[tunnel].tunnels_to.iter() {
                check.insert(neighbor.clone());
            }
        }
        tunnels = check.into_iter().collect();
        // tunnels = Vec::new();
        // for tunnel_to_check in check {
        //     tunnels.push(tunnel_to_check.clone());
        // }
        depth += 1;
    }
}
//     let mut depth = 0;
//     let mut tunnels: Vec<String> = vec![start.clone()];
//     loop {
//         println!("{}>{}, Depth: {}", start.as_str(), end.as_str(), depth);
//         println!("{:?}", tunnels);
//         visited.insert(start.clone());
//         depth += 1;
//         let mut tunnels_to_try: Vec<String> = Vec::new();
//         for tunnel in &tunnels {
//             let connected_tunnels = &valves[tunnel];
//             for connected_tunnel in connected_tunnels.tunnels_to.iter() {
//                 if visited.contains(connected_tunnel) {
//                     continue;
//                 }
//                 visited.insert(tunnel.clone());
//                 tunnels_to_try.push(tunnel.clone());
//             }
//         }
//         for tunnel in tunnels_to_try.iter() {
//             if tunnel.clone() == end.clone() {
//                 return Some(depth);
//             } else if !visited.contains(tunnel) {
//                 visited.insert(tunnel.clone());
//                 tunnels.push(tunnel.clone());
//             }
//         }
//         // tunnels = tunnels_to_try.clone();
//     }
//     //
// }
// ) -> (Option<usize>, Vec<String>) {
// // println!("  DIST {} > {}", &start, &end);
// let start_valve = &valves[&start];
// let return_path: Vec<String> = Vec::new();
// // let end_valve = valves[&end];
// let mut shortest_distance: Option<usize> = None;
// let mut fastest_tunnel = "".to_owned();
// for tunnel in start_valve.tunnels_to.iter() {
//     if tunnel.clone() == end.clone() {
//         // println!("Found {}", tunnel);
//         return_path.push(tunnel.clone());
//         return (Some(1usize), return_path);
//     }
//     if visited.contains(tunnel) {
//         continue;
//     }
//     visited.insert(tunnel.clone());
//     let (distance, path) = get_shortest_distance(tunnel.clone(), end.clone(), valves, visited);
//     match distance {
//         None => {
//             continue;
//         }
//         Some(valid_distance) => {
//             if shortest_distance.is_none() || valid_distance < shortest_distance.unwrap() {
//                 shortest_distance = Some(valid_distance);
//                 fastest_tunnel = tunnel.clone();
//             }
//         }
//     }
// }

// println!("Fastest tunnel: {}", fastest_tunnel);

// if let Some(valid_distance) = shortest_distance {
//     Some(valid_distance + 1)
// } else {
//     None
// }
// }

#[derive(Clone)]
struct Player {
    score: u64,
    position: String,
    moves_remaining: usize,
    valves_remaining: Vec<String>,
    history: Vec<String>,
}
impl Player {
    fn new(position: String, moves_remaining: usize, valuable_valves: &HashSet<String>) -> Self {
        let mut valves_remaining = Vec::new();
        for valuable in valuable_valves {
            valves_remaining.push(valuable.clone());
        }
        Self {
            score: 0u64,
            position,
            moves_remaining,
            valves_remaining,
            history: Vec::new(),
        }
    }
}

fn score_player(
    player: &Player,
    distance_map: &DistanceMap,
    valves: &HashMap<String, Valve>,
) -> Player {
    // println!(
    //     "This is to check for negatives: {}",
    //     &player.moves_remaining
    // );
    if player.moves_remaining == 0 {
        let mut return_player = player.clone();
        return_player
            .history
            .push(format!("Exhausted with {}", &return_player.score));

        // for hist in &return_player.history {
        //     println!("{}", hist);
        // }
        // println!("\n");
        return return_player;
    }

    if player.valves_remaining.len() == 0 {
        // println!("Finished!  Score: {}", player.score);
        let mut return_player = player.clone();
        return_player
            .history
            .push(format!("Finished with {}", &return_player.score));
        // seek out the example's best winner...
        // if player.history[0].ends_with(" at valve DD") {
        //     if player.history[1].ends_with(" at valve BB") {
        //         if player.history[2].ends_with(" at valve JJ") {
        //             for hist in &return_player.history {
        //                 println!("{}", hist);
        //             }
        //             println!("\n");
        //         }
        //     }
        // }
        return return_player;
    }

    let mut next_round_players: Vec<Player> = Vec::new();

    for next_valve in player.valves_remaining.iter() {
        // if next_valve.clone() == player.position.clone() {
        //     continue;
        // }
        let movement_key =
            DistanceMap::key_from_strs(&player.position.as_str(), &next_valve.as_str());
        // need +1 to turn the valve
        let distance_traveled = distance_map.get_and_unwrap(&movement_key) + 1;
        if distance_traveled > player.moves_remaining {
            // you can't reach it, or if you can you won't turn it...
            continue;
        }
        let moves_remaining = player.moves_remaining - distance_traveled;
        let gained_score = score_valve(&valves[next_valve], moves_remaining);
        let score = player.score + gained_score;
        let valves_remaining = player
            .valves_remaining
            .iter()
            .filter(|item| item.as_str() != next_valve.as_str())
            .map(|item| item.to_owned())
            .collect();
        let position = next_valve.clone();

        let mut new_player = Player {
            score,
            position: position.clone(),
            moves_remaining,
            valves_remaining,
            history: player.history.clone(),
        };
        new_player.history.push(format!(
            "I scored {}x{}={} at valve {}",
            valves[&position].flow_rate,
            moves_remaining + 0,
            gained_score,
            position
        ));
        new_player
            .history
            .push(format!("I have {} moves left.", moves_remaining));
        next_round_players.push(new_player);
    }

    let mut greatest_score = 0u64;
    let mut best_player = player.clone();

    if next_round_players.len() == 0 {
        best_player.history.push("I'm out of options!".to_owned());
        return best_player;
    }

    for new_player in &next_round_players {
        let test_player = score_player(new_player, distance_map, valves);
        if test_player.score > greatest_score {
            greatest_score = test_player.score;
            best_player = test_player.clone();
        }
    }

    // println!("Next round: {} players", next_round_players.len());

    best_player
}

fn score_valve(valve: &Valve, moves_scored: usize) -> u64 {
    valve.flow_rate * (moves_scored + 0) as u64
}

pub fn d16s1(submit: bool) {
    let (valves, _first_name) = input();
    // let first_valve = valves.get(&first_name).unwrap();
    // let root_node = ChoiceNode::new(HashSet::new(), first_valve, 0, 0);
    let first_name = "AA".to_owned();
    let mut valuable_valves: HashSet<String> = HashSet::new();
    for (name, valve) in &valves {
        if valve.flow_rate > 0 {
            valuable_valves.insert(name.clone());
        }
    }
    let mut traveling_starting_points = valuable_valves.clone();
    traveling_starting_points.insert(first_name.clone());

    // println!("START: {:?}", &traveling_starting_points);
    // println!("ENDS : {:?}", &valuable_valves);
    let mut distance_map: DistanceMap = DistanceMap::new();
    for start in traveling_starting_points.iter() {
        for end in valuable_valves.iter() {
            if start == end {
                continue;
            }

            let mut visited: HashSet<String> = HashSet::new();
            // println!("DIST {} > {}", &start, &end);
            let distance = get_shortest_distance(start.clone(), end.clone(), &valves, &mut visited);
            let key = DistanceMap::key_from_strs(start.as_str(), end.as_str());
            distance_map.insert(
                key.clone(),
                distance.expect(format!("Somehow there's no valid path for {}", &key).as_str()),
            );
        }
    }
    println!("DISTANCES:\n{:?}", distance_map.data);

    println!("\n\n");
    let mut visited: HashSet<String> = HashSet::new();
    // println!("DIST {} > {}", &start, &end);
    // let distance = get_shortest_distance("BB".to_owned(), "JJ".to_owned(), &valves, &mut visited);
    // println!("Distance: {}", distance.unwrap());

    // let mut tree_root = ChoiceTree::new(0usize, root_node);
    // let answer = find_the_most_pressure(30, &mut tree_root, &valves);
    println!("\n\n");

    let move_count = 30;

    let root = Player::new(first_name.clone(), move_count, &valuable_valves);
    let best_player = score_player(&root, &distance_map, &valves);
    for hist in best_player.history {
        println!("{}", hist);
    }
    let answer = best_player.score;

    final_answer(answer, submit, DAY, 1);
}

pub fn d16s2(_submit: bool) {}
