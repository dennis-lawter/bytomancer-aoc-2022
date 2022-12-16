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

struct DistanceMap {
    data: HashMap<String, usize>,
}
impl DistanceMap {
    fn key_from_strs(from: &str, to: &str) -> String {
        format!("{}>{}", from, to)
    }

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

        depth += 1;
    }
}

#[derive(Clone)]
struct Player {
    score: u64,
    position: String,
    helper: Option<String>,
    moves_remaining: usize,
    helper_moves_remaining: Option<usize>,
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
            helper: None,
            moves_remaining,
            helper_moves_remaining: None,
            valves_remaining,
            history: Vec::new(),
        }
    }
    fn new_with_helper(
        position: String,
        moves_remaining: usize,
        valuable_valves: &HashSet<String>,
        helper: String,
        helper_moves_remaining: usize,
    ) -> Self {
        let mut valves_remaining = Vec::new();
        for valuable in valuable_valves {
            valves_remaining.push(valuable.clone());
        }
        Self {
            score: 0u64,
            position,
            helper: Some(helper),
            moves_remaining,
            helper_moves_remaining: Some(helper_moves_remaining),
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
    if player.moves_remaining == 0 {
        let mut return_player = player.clone();
        return_player
            .history
            .push(format!("Exhausted with {}", &return_player.score));

        return return_player;
    }

    if player.valves_remaining.len() == 0 {
        let mut return_player = player.clone();
        return_player
            .history
            .push(format!("Finished with {}", &return_player.score));

        return return_player;
    }

    let mut next_round_players: Vec<Player> = Vec::new();

    for next_valve in player.valves_remaining.iter() {
        match move_player(player, next_valve, valves, distance_map) {
            Some(new_player) => {
                next_round_players.push(new_player);
            }
            None => {}
        }
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

    best_player
}

fn move_player(
    player: &Player,
    next_valve_for_player: &String,
    valves: &HashMap<String, Valve>,
    distance_map: &DistanceMap,
) -> Option<Player> {
    let player_movement_key =
        DistanceMap::key_from_strs(&player.position.as_str(), next_valve_for_player.as_str());
    // need +1 to turn the valve
    let distance_traveled = distance_map.get_and_unwrap(&player_movement_key) + 1;
    if distance_traveled > player.moves_remaining {
        // you can't reach it, or if you can you won't turn it...
        return None;
    }
    let moves_remaining = player.moves_remaining - distance_traveled;
    let gained_score = score_valve(&valves[next_valve_for_player], moves_remaining);
    let score = player.score + gained_score;
    let valves_remaining = player
        .valves_remaining
        .iter()
        .filter(|item| item.as_str() != next_valve_for_player.as_str())
        .map(|item| item.to_owned())
        .collect();
    let position = next_valve_for_player.clone();

    let mut new_player = Player {
        score,
        position: position.clone(),
        helper: player.helper.clone(),
        moves_remaining,
        helper_moves_remaining: player.helper_moves_remaining,
        valves_remaining,
        history: player.history.clone(),
    };
    new_player.history.push(format!(
        "I scored {}x{}={} at valve {}",
        valves[&position].flow_rate, moves_remaining, gained_score, position
    ));
    new_player
        .history
        .push(format!("I have {} moves left.", moves_remaining));

    Some(new_player)
}

fn move_helper(
    player: &Player,
    next_valve_for_player: &String,
    valves: &HashMap<String, Valve>,
    distance_map: &DistanceMap,
) -> Option<Player> {
    let helper_position = player.helper.as_ref().unwrap();
    let helper_movement_key =
        DistanceMap::key_from_strs(helper_position.as_str(), next_valve_for_player.as_str());
    // need +1 to turn the valve
    let distance_traveled = distance_map.get_and_unwrap(&helper_movement_key) + 1;
    if distance_traveled > player.helper_moves_remaining.unwrap() {
        // you can't reach it, or if you can you won't turn it...
        return None;
    }
    let helper_moves_remaining = player.helper_moves_remaining.unwrap() - distance_traveled;
    let gained_score = score_valve(&valves[next_valve_for_player], helper_moves_remaining);
    let score = player.score + gained_score;
    let valves_remaining = player
        .valves_remaining
        .iter()
        .filter(|item| item.as_str() != next_valve_for_player.as_str())
        .map(|item| item.to_owned())
        .collect();
    let helper = next_valve_for_player.clone();

    let mut new_player = Player {
        score,
        position: player.position.clone(),
        helper: Some(helper.clone()),
        moves_remaining: player.moves_remaining,
        helper_moves_remaining: Some(helper_moves_remaining),
        valves_remaining,
        history: player.history.clone(),
    };
    new_player.history.push(format!(
        "Helper scored {}x{}={} at valve {}",
        valves[&helper].flow_rate, helper_moves_remaining, gained_score, helper
    ));
    new_player
        .history
        .push(format!("Helper has {} moves left.", helper_moves_remaining));

    Some(new_player)
}

fn score_player_with_helper(
    player: &Player,
    distance_map: &DistanceMap,
    valves: &HashMap<String, Valve>,
) -> Player {
    if player.moves_remaining == 0 && player.helper_moves_remaining.unwrap() == 0 {
        let mut return_player = player.clone();
        // println!(
        //     "Exhausted, player at {} helper at {}, score was {}",
        //     player.position.as_str(),
        //     player.helper.as_ref().unwrap().as_str(),
        //     player.score
        // );
        return_player
            .history
            .push(format!("Exhausted with {}", &return_player.score));

        return return_player;
    }

    if player.valves_remaining.len() == 0 {
        let mut return_player = player.clone();
        // println!(
        //     "Finished, player at {} helper at {}, score was {}",
        //     player.position.as_str(),
        //     player.helper.as_ref().unwrap().as_str(),
        //     player.score
        // );
        return_player
            .history
            .push(format!("Finished with {}", &return_player.score));

        return return_player;
    }

    let mut next_round_players: Vec<Player> = Vec::new();

    for next_valve in player.valves_remaining.iter() {
        match move_player(player, next_valve, valves, distance_map) {
            Some(new_player) => {
                next_round_players.push(new_player);
            }
            None => {}
        }
        match move_helper(player, next_valve, valves, distance_map) {
            Some(new_player) => {
                next_round_players.push(new_player);
            }
            None => {}
        }
    }

    let mut greatest_score = 0u64;
    let mut best_player = player.clone();

    if next_round_players.len() == 0 {
        // println!(
        //     "Options ran out, player at {} helper at {}, score was {}",
        //     player.position.as_str(),
        //     player.helper.as_ref().unwrap().as_str(),
        //     player.score
        // );
        best_player.history.push("I'm out of options!".to_owned());
        return best_player;
    }

    for new_player in &next_round_players {
        let test_player = score_player_with_helper(new_player, distance_map, valves);
        if test_player.score > greatest_score {
            greatest_score = test_player.score;
            best_player = test_player.clone();
        }
    }

    // println!(
    //     "BEST SCORE OF {} PLAYERS WAS {}",
    //     next_round_players.len(),
    //     greatest_score
    // );

    best_player
}

fn score_valve(valve: &Valve, moves_scored: usize) -> u64 {
    valve.flow_rate * (moves_scored + 0) as u64
}

pub fn d16s1(submit: bool) {
    let (valves, _first_name) = input();
    let first_name = "AA".to_owned();
    let mut valuable_valves: HashSet<String> = HashSet::new();
    for (name, valve) in &valves {
        if valve.flow_rate > 0 {
            valuable_valves.insert(name.clone());
        }
    }
    let mut traveling_starting_points = valuable_valves.clone();
    traveling_starting_points.insert(first_name.clone());

    let mut distance_map: DistanceMap = DistanceMap::new();
    for start in traveling_starting_points.iter() {
        for end in valuable_valves.iter() {
            if start == end {
                continue;
            }

            let mut visited: HashSet<String> = HashSet::new();
            let distance = get_shortest_distance(start.clone(), end.clone(), &valves, &mut visited);
            let key = DistanceMap::key_from_strs(start.as_str(), end.as_str());
            distance_map.insert(
                key.clone(),
                distance.expect(format!("Somehow there's no valid path for {}", &key).as_str()),
            );
        }
    }

    let move_count = 30;

    let root = Player::new(first_name.clone(), move_count, &valuable_valves);
    let best_player = score_player(&root, &distance_map, &valves);
    for hist in best_player.history {
        println!("{}", hist);
    }
    let answer = best_player.score;

    final_answer(answer, submit, DAY, 1);
}

pub fn d16s2(submit: bool) {
    let (valves, _first_name) = input();
    let first_name = "AA".to_owned();
    let mut valuable_valves: HashSet<String> = HashSet::new();
    for (name, valve) in &valves {
        if valve.flow_rate > 0 {
            valuable_valves.insert(name.clone());
        }
    }
    let mut traveling_starting_points = valuable_valves.clone();
    traveling_starting_points.insert(first_name.clone());

    let mut distance_map: DistanceMap = DistanceMap::new();
    for start in traveling_starting_points.iter() {
        for end in valuable_valves.iter() {
            if start == end {
                continue;
            }

            let mut visited: HashSet<String> = HashSet::new();
            let distance = get_shortest_distance(start.clone(), end.clone(), &valves, &mut visited);
            let key = DistanceMap::key_from_strs(start.as_str(), end.as_str());
            distance_map.insert(
                key.clone(),
                distance.expect(format!("Somehow there's no valid path for {}", &key).as_str()),
            );
        }
    }

    let move_count = 13;

    let root = Player::new_with_helper(
        first_name.clone(),
        move_count,
        &valuable_valves,
        first_name.clone(),
        move_count,
    );
    let best_player = score_player_with_helper(&root, &distance_map, &valves);
    for hist in best_player.history {
        println!("{}", hist);
    }
    let answer = best_player.score;

    final_answer(answer, submit, DAY, 2);
}
