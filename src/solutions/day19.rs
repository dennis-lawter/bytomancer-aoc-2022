use std::cmp::max;

use indexmap::IndexMap;
use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 19;

struct RoboBlueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

fn input() -> IndexMap<u32, RoboBlueprint> {
    let raw = input_raw(DAY);
    let lines: Vec<&str> = raw.split("\n").collect();
    let mut result: IndexMap<u32, RoboBlueprint> = IndexMap::new();

    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    for line in lines {
        let captures = regex.captures(line).unwrap();
        let bp = RoboBlueprint {
            id: str::parse::<u32>(captures.get(1).unwrap().as_str()).unwrap(),
            ore_robot_ore_cost: str::parse::<u32>(captures.get(2).unwrap().as_str()).unwrap(),
            clay_robot_ore_cost: str::parse::<u32>(captures.get(3).unwrap().as_str()).unwrap(),
            obsidian_robot_ore_cost: str::parse::<u32>(captures.get(4).unwrap().as_str()).unwrap(),
            obsidian_robot_clay_cost: str::parse::<u32>(captures.get(5).unwrap().as_str()).unwrap(),
            geode_robot_ore_cost: str::parse::<u32>(captures.get(6).unwrap().as_str()).unwrap(),
            geode_robot_obsidian_cost: str::parse::<u32>(captures.get(7).unwrap().as_str())
                .unwrap(),
        };
        result.insert(bp.id, bp);
    }

    result
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct RoboState {
    num_ore_robots: u32,
    num_clay_robots: u32,
    num_obsidian_robots: u32,
    num_geode_robots: u32,

    num_ore: u32,
    num_clay: u32,
    num_obsidian: u32,
    num_geodes: u32,

    turns_remaining: u32,
    blueprint_id: u32,

    history: Vec<String>,
}
impl std::fmt::Display for RoboState {
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     write!(f, "num_ore_robots: {},\nnum_clay_robots: {},\nnum_obsidian_robots: {},\nnum_geode_robots: {},\n\nnum_ore: {},\nnum_clay: {},\nnum_obsidian: {},\nnum_geodes: {},\n\nturns_remaining: {},\nblueprint_id: {},",

    // self.num_ore_robots,
    // self.num_clay_robots,
    // self.num_obsidian_robots,
    // self.num_geode_robots,

    // self.num_ore,
    // self.num_clay,
    // self.num_obsidian,
    // self.num_geodes,

    // self.turns_remaining,
    // self.blueprint_id,)
    // }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} geo, {} remain: {}",
            self.num_geodes,
            self.turns_remaining,
            self.history.join(" ")
        )
    }
}
impl RoboState {
    fn new(turns_remaining: u32, blueprint_id: u32) -> Self {
        Self {
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
            num_ore: 0,
            num_clay: 0,
            num_obsidian: 0,
            num_geodes: 0,

            turns_remaining,
            blueprint_id,

            history: Vec::new(),
        }
    }
    fn can_afford_ore_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_ore >= blueprints[&self.blueprint_id].ore_robot_ore_cost
    }
    fn needs_ore_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        let max_ore = max(
            blueprints[&self.blueprint_id].ore_robot_ore_cost,
            max(
                blueprints[&self.blueprint_id].clay_robot_ore_cost,
                max(
                    blueprints[&self.blueprint_id].obsidian_robot_ore_cost,
                    blueprints[&self.blueprint_id].geode_robot_ore_cost,
                ),
            ),
        );
        self.num_ore_robots < max_ore
    }
    fn buy_ore_robot(&mut self, blueprints: &IndexMap<u32, RoboBlueprint>) {
        self.num_ore -= blueprints[&self.blueprint_id].ore_robot_ore_cost;
        self.num_ore_robots += 1;
        self.history
            .push(format!("[ore @ {}]", self.turns_remaining));
    }
    fn can_afford_clay_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_ore >= blueprints[&self.blueprint_id].clay_robot_ore_cost
    }
    fn needs_clay_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_clay_robots < blueprints[&self.blueprint_id].obsidian_robot_clay_cost
    }
    fn buy_clay_robot(&mut self, blueprints: &IndexMap<u32, RoboBlueprint>) {
        self.num_ore -= blueprints[&self.blueprint_id].clay_robot_ore_cost;
        self.num_clay_robots += 1;
        self.history
            .push(format!("[clay @ {}]", self.turns_remaining));
    }
    fn can_afford_obsidian_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_ore >= blueprints[&self.blueprint_id].obsidian_robot_ore_cost
            && self.num_clay >= blueprints[&self.blueprint_id].obsidian_robot_clay_cost
    }
    fn needs_obsidian_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_obsidian_robots < blueprints[&self.blueprint_id].geode_robot_obsidian_cost
    }
    fn buy_obsidian_robot(&mut self, blueprints: &IndexMap<u32, RoboBlueprint>) {
        self.num_ore -= blueprints[&self.blueprint_id].obsidian_robot_ore_cost;
        self.num_clay -= blueprints[&self.blueprint_id].obsidian_robot_clay_cost;
        self.num_obsidian_robots += 1;
        self.history
            .push(format!("[obsidian @ {}]", self.turns_remaining));
    }
    fn could_afford_obsidian_robot(&self) -> bool {
        self.num_clay_robots > 0
    }
    fn can_afford_geode_robot(&self, blueprints: &IndexMap<u32, RoboBlueprint>) -> bool {
        self.num_ore >= blueprints[&self.blueprint_id].geode_robot_ore_cost
            && self.num_obsidian >= blueprints[&self.blueprint_id].geode_robot_obsidian_cost
    }
    fn could_afford_geode_robot(&self) -> bool {
        self.num_obsidian_robots > 0
    }
    fn buy_geode_robot(&mut self, blueprints: &IndexMap<u32, RoboBlueprint>) {
        self.num_ore -= blueprints[&self.blueprint_id].geode_robot_ore_cost;
        self.num_obsidian -= blueprints[&self.blueprint_id].geode_robot_obsidian_cost;
        self.num_geode_robots += 1;
        self.history
            .push(format!("[GEODE @ {}]", self.turns_remaining));
    }
    fn tick(&mut self) {
        if self.turns_remaining > 0 {
            self.num_ore += self.num_ore_robots;
            self.num_clay += self.num_clay_robots;
            self.num_obsidian += self.num_obsidian_robots;
            self.num_geodes += self.num_geode_robots;
            self.turns_remaining -= 1;
            // if self.num_geodes > 9 {
            //     println!("{:?}\n\n", self);
            // }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Intent {
    BuyOreRobot,
    BuyClayRobot,
    BuyObsidianRobot,
    BuyGeodeRobot,
}

fn rate_state_quality(
    state: RoboState,
    blueprints: &IndexMap<u32, RoboBlueprint>,
    earliest_clay: &mut u32,
    earliest_obsidian: &mut u32,
    earliest_geode: &mut u32,
) -> RoboState {
    // std::thread::sleep(core::time::Duration::from_millis(1000));
    // println!("STATE:\n{}", state);
    if state.turns_remaining == 0 {
        // println!("EXHAUSTED\n{}", state);
        return state;
    }
    let mut states_to_try: IndexMap<Intent, RoboState> = IndexMap::new();
    if state.could_afford_geode_robot() {
        let mut next_state = state.clone();
        while !next_state.can_afford_geode_robot(blueprints) && next_state.turns_remaining > 0 {
            next_state.tick();
        }
        if next_state.can_afford_geode_robot(blueprints) {
            // println!(
            //     "BUYING GEODE BOT, Turns remaining... {}",
            //     next_state.turns_remaining
            // );

            next_state.tick();
            next_state.buy_geode_robot(blueprints);

            states_to_try.insert(Intent::BuyGeodeRobot, next_state);
        }
    }
    if state.could_afford_obsidian_robot() && state.needs_obsidian_robot(blueprints) {
        let mut next_state = state.clone();
        while !next_state.can_afford_obsidian_robot(blueprints) && next_state.turns_remaining > 0 {
            next_state.tick();
        }
        if next_state.can_afford_obsidian_robot(blueprints) {
            // println!(
            //     "BUYING OBSIDIAN BOT, Turns remaining... {}",
            //     next_state.turns_remaining
            // );
            next_state.tick();
            next_state.buy_obsidian_robot(blueprints);

            states_to_try.insert(Intent::BuyObsidianRobot, next_state);
        }
    }
    if state.needs_clay_robot(blueprints) {
        let mut next_state = state.clone();
        while !next_state.can_afford_clay_robot(blueprints) && next_state.turns_remaining > 0 {
            next_state.tick();
        }
        if next_state.can_afford_clay_robot(blueprints) {
            // println!(
            //     "BUYING CLAY BOT, Turns remaining... {}",
            //     next_state.turns_remaining
            // );

            next_state.tick();
            next_state.buy_clay_robot(blueprints);

            states_to_try.insert(Intent::BuyClayRobot, next_state);
        }
    }
    if state.needs_ore_robot(blueprints) {
        let mut next_state = state.clone();
        while !next_state.can_afford_ore_robot(blueprints) && next_state.turns_remaining > 0 {
            next_state.tick();
        }
        if next_state.can_afford_ore_robot(blueprints) {
            // println!(
            //     "BUYING ORE BOT, Turns remaining... {}",
            //     next_state.turns_remaining
            // );
            next_state.tick();
            next_state.buy_ore_robot(blueprints);
            states_to_try.insert(Intent::BuyOreRobot, next_state);
        }
    }
    if states_to_try.len() == 0 {
        // there will be no more purchases, we can end this simulation
        let mut next_state = state.clone();
        while next_state.turns_remaining > 0 {
            next_state.tick();
        }
        // println!("Giving up, but I collected {}", next_state.num_geodes);
        // println!("NO OPTIONS\n{}", next_state);
        return next_state;
    }
    // max quality observed
    let mut max_quality = 0u32;
    let mut best_state = state.clone();
    for (_intent, next_state) in states_to_try.iter_mut() {
        let state_assessment = rate_state_quality(
            next_state.clone(),
            blueprints,
            earliest_clay,
            earliest_obsidian,
            earliest_geode,
        );
        if state_assessment.num_geodes > max_quality {
            max_quality = state_assessment.num_geodes;
            best_state = state_assessment;
        }
    }

    best_state
}

pub fn d19s1(submit: bool) {
    let blueprints = input();
    const MINUTES: u32 = 24;

    let mut answer = 0u32;

    for (id, blueprint) in blueprints.iter() {
        // MINUTES may need adjustment?
        let state = RoboState::new(MINUTES + 0, blueprint.id);
        let mut earliest_clay = 0u32;
        let mut earliest_obsidian = 0u32;
        let mut earliest_geode = 0u32;
        let best_state = rate_state_quality(
            state.clone(),
            &blueprints,
            &mut earliest_clay,
            &mut earliest_obsidian,
            &mut earliest_geode,
        );
        // quality += 1; // wtf?
        println!("Quality found for BP#{}: {}", id, best_state.num_geodes);
        println!("STATE:\n{}", best_state);
        answer += best_state.num_geodes * id;
    }

    final_answer(answer, submit, DAY, 1);
}

pub fn d19s2(submit: bool) {
    let blueprints = input();
    const MINUTES: u32 = 32;

    let mut answers: Vec<u32> = Vec::new();

    for (id, blueprint) in blueprints.iter() {
        if *id > 3 {
            break;
        }
        // MINUTES may need adjustment?
        let state = RoboState::new(MINUTES + 0, blueprint.id);
        let mut earliest_clay = 0u32;
        let mut earliest_obsidian = 0u32;
        let mut earliest_geode = 0u32;
        let best_state = rate_state_quality(
            state.clone(),
            &blueprints,
            &mut earliest_clay,
            &mut earliest_obsidian,
            &mut earliest_geode,
        );
        // quality += 1; // wtf?
        println!("Quality found for BP#{}: {}", id, best_state.num_geodes);
        println!("STATE:\n{}", best_state);
        answers.push(best_state.num_geodes);
    }
    let answer = answers[0] * answers[1] * answers.get(2).or(Some(&1u32)).unwrap();
    final_answer(answer, submit, DAY, 2);
}
