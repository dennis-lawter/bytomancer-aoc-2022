use std::char::MAX;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 12;

#[derive(Clone, PartialEq, PartialOrd, Eq)]
struct SearchNode {
    position: (usize, usize),
    // previous: Option<(usize, usize)>,
    // next_choices: Vec<(usize, usize)>,
    previous: Vec<(usize, usize)>,
    goal: (usize, usize),
    height: u8,
}
impl SearchNode {
    pub fn new(
        position: (usize, usize),
        previous: Vec<(usize, usize)>,
        goal: (usize, usize),
        height: u8,
    ) -> Self {
        Self {
            position,
            previous,
            goal,
            height,
        }
    }
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_d = get_distance(self.position, self.goal);
        let other_d = get_distance(other.position, other.goal);
        match self_d.cmp(&other_d) {
            std::cmp::Ordering::Equal => self.height.cmp(&other.height),
            inequality => inequality,
        }
    }
}

fn get_distance(pos: (usize, usize), goal: (usize, usize)) -> usize {
    let x: usize;
    if goal.0 > pos.0 {
        x = goal.0 - pos.0
    } else {
        x = pos.0 - goal.0
    }
    let y: usize;
    if goal.1 > pos.1 {
        y = goal.1 - pos.1
    } else {
        y = pos.1 - goal.1
    }

    x + y
}

fn shortest_path_distance(nodes: Vec<SearchNode>, board: &mut Board) -> usize {
    let shortest_path_distance = 0usize;

    let mut cloned_nodes = nodes.clone();
    cloned_nodes.sort_unstable();
    cloned_nodes.reverse();

    loop {}

    shortest_path_distance
}

struct Board {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    starting_pos: (usize, usize),
    ending_pos: (usize, usize),
}

fn input() -> Board {
    let raw = input_raw(DAY);
    let raw_split = raw.split("\n");
    let lines: Vec<&str> = raw_split.collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut map = Vec::with_capacity(height);
    let mut explored = Vec::with_capacity(height);
    let mut starting_pos = (0usize, 0usize);
    let mut ending_pos = (0usize, 0usize);
    for i in 0..height {
        let mut map_row: Vec<u8> = lines[i]
            .bytes()
            .map(|cur_char| match cur_char {
                69u8 => 27u8,
                83u8 => 0u8,
                regular_char => regular_char + 1 - ('a' as u8),
            })
            .collect();
        let mut exp_row = vec![false; width];
        for j in 0..width {
            if map_row[j] == 0 {
                starting_pos = (j, i);
                exp_row[j] = true;
            } else if map_row[j] == 27 {
                ending_pos = (j, i);
            }
        }
        map.push(map_row);
        explored.push(exp_row);
    }
    Board {
        map,
        explored,
        width,
        height,
        starting_pos,
        ending_pos,
    }
}

pub fn d12s1(submit: bool) {
    let mut board = input();

    let root = SearchNode::new(board.starting_pos, Vec::new(), board.ending_pos, 0);
    // let tree = vec![root];

    let depth = shortest_path_distance(vec![root], &mut board);

    final_answer(depth, submit, DAY, 1);
}

pub fn d12s2(submit: bool) {
    let input = input();
    // final_answer(input.len(), submit, DAY, 2);
}
