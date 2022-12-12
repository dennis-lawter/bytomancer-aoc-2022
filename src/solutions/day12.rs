use std::char::MAX;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 12;

#[derive(Clone, PartialEq, PartialOrd, Eq)]
struct BfsNode {
    position: (usize, usize),
    // previous: Option<(usize, usize)>,
    // next_choices: Vec<(usize, usize)>,
    previous: Vec<(usize, usize)>,
    goal: (usize, usize),
    height: u8,
}
impl BfsNode {
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
    // pub fn add_choice(&mut self, new_pos: (usize, usize)) {
    //     if !self.previous.contains(new_pos) {
    //         self.next_choices.push(new_pos);
    //     }
    // }
}
impl Ord for BfsNode {
    // fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //     let self_d = get_distance(self.position, self.goal);
    //     let other_d = get_distance(other.position, other.goal);
    //     match self_d.cmp(&other_d) {
    //         std::cmp::Ordering::Equal => self.height.cmp(&other.height),
    //         inequality => inequality,
    //     }
    // }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.height.cmp(&other.height) {
            std::cmp::Ordering::Equal => {
                let self_d = get_distance(self.position, self.goal);
                let other_d = get_distance(other.position, other.goal);
                return self_d.cmp(&other_d);
            }
            inequality => inequality,
        }
    }
}

struct Board {
    map: Vec<Vec<u8>>,
    explored: Vec<Vec<bool>>,
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

// fn bfs_depth(nodes: Vec<BfsNode>, board: &mut Board, depth: usize) -> usize {
//     print!("Depth: {}", depth);
//     println!("\tNodes: {}", nodes.len());
//     let mut nodes_to_try: Vec<BfsNode> = Vec::new();
//     for node in nodes {
//         // board.explored[node.position.1][node.position.0] = true;
//         let current_height = board.map[node.position.1][node.position.0];
//         let mut previous = node.previous;
//         previous.push(node.position);
//         if node.position == board.ending_pos {
//             return depth;
//         }
//         if node.position.0 > 0 {
//             let sample_position = (node.position.0 - 1, node.position.1);
//             // if board.explored[sample_position.1][sample_position.0] == false {
//             if !previous.contains(&sample_position) {
//                 if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                     nodes_to_try.push(BfsNode::new(
//                         sample_position,
//                         previous.clone(),
//                         board.ending_pos,
//                     ));
//                 }
//             }
//         }
//         if node.position.0 < board.width - 1 {
//             let sample_position = (node.position.0 + 1, node.position.1);
//             // if board.explored[sample_position.1][sample_position.0] == false {
//             if !previous.contains(&sample_position) {
//                 if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                     nodes_to_try.push(BfsNode::new(
//                         sample_position,
//                         previous.clone(),
//                         board.ending_pos,
//                     ));
//                 }
//             }
//         }
//         if node.position.1 > 0 {
//             let sample_position = (node.position.0, node.position.1 - 1);
//             // if board.explored[sample_position.1][sample_position.0] == false {
//             if !previous.contains(&sample_position) {
//                 if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                     nodes_to_try.push(BfsNode::new(
//                         sample_position,
//                         previous.clone(),
//                         board.ending_pos,
//                     ));
//                 }
//             }
//         }
//         if node.position.1 < board.height - 1 {
//             let sample_position = (node.position.0, node.position.1 + 1);
//             // if board.explored[sample_position.1][sample_position.0] == false {
//             if !previous.contains(&sample_position) {
//                 if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                     nodes_to_try.push(BfsNode::new(
//                         sample_position,
//                         previous.clone(),
//                         board.ending_pos,
//                     ));
//                 }
//             }
//         }
//     }

//     bfs_depth(nodes_to_try, board, depth + 1)
// }

// fn bfs_depth_flat(node: BfsNode, board: &mut Board) -> usize {
//     let mut depth = 0usize;
//     let mut nodes = vec![node];
//     loop {
//         print!("Depth: {}", depth);
//         println!("\tNodes: {}", nodes.len());
//         let mut nodes_to_try: Vec<BfsNode> = Vec::new();
//         for node in nodes {
//             // board.explored[node.position.1][node.position.0] = true;
//             let current_height = board.map[node.position.1][node.position.0];
//             let mut previous = node.previous;
//             previous.push(node.position);
//             if node.position == board.ending_pos {
//                 return depth;
//             }
//             if node.position.0 > 0 {
//                 let sample_position = (node.position.0 - 1, node.position.1);
//                 // if board.explored[sample_position.1][sample_position.0] == false {
//                 if !previous.contains(&sample_position) {
//                     if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                         nodes_to_try.push(BfsNode::new(
//                             sample_position,
//                             previous.clone(),
//                             board.ending_pos,
//                         ));
//                     }
//                 }
//             }
//             if node.position.0 < board.width - 1 {
//                 let sample_position = (node.position.0 + 1, node.position.1);
//                 // if board.explored[sample_position.1][sample_position.0] == false {
//                 if !previous.contains(&sample_position) {
//                     if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                         nodes_to_try.push(BfsNode::new(
//                             sample_position,
//                             previous.clone(),
//                             board.ending_pos,
//                         ));
//                     }
//                 }
//             }
//             if node.position.1 > 0 {
//                 let sample_position = (node.position.0, node.position.1 - 1);
//                 // if board.explored[sample_position.1][sample_position.0] == false {
//                 if !previous.contains(&sample_position) {
//                     if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                         nodes_to_try.push(BfsNode::new(
//                             sample_position,
//                             previous.clone(),
//                             board.ending_pos,
//                         ));
//                     }
//                 }
//             }
//             if node.position.1 < board.height - 1 {
//                 let sample_position = (node.position.0, node.position.1 + 1);
//                 // if board.explored[sample_position.1][sample_position.0] == false {
//                 if !previous.contains(&sample_position) {
//                     if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
//                         nodes_to_try.push(BfsNode::new(
//                             sample_position,
//                             previous.clone(),
//                             board.ending_pos,
//                         ));
//                     }
//                 }
//             }
//         }
//         nodes = nodes_to_try;
//         depth += 1;
//     }
// }

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

const MAX_NODES: usize = 5_000_000;
// const MAX_NODES: usize = 1_000_000;
// const MAX_NODES: usize = 200_000;
// const MAX_NODES: usize = 60;

fn heuristic(nodes: Vec<BfsNode>, _goal: (usize, usize)) -> Vec<BfsNode> {
    // let mut new_vec = Vec::new();
    // let mut closest_distance = get_distance(nodes[0].position, goal);
    // for node in nodes.clone() {
    //     if get_distance(node.position, goal) < closest_distance {
    //         closest_distance = get_distance(node.position, goal);
    //     }
    // }
    // println!("  Closest distance: {}", closest_distance);
    // for node in nodes {
    //     // if closest_distance > 5 {
    //     if get_distance(node.position, goal) <= closest_distance + 3 {
    //         // println!(
    //         //     "  Good enough: {} from {:?}",
    //         //     get_distance(node.position, goal),
    //         //     node.position
    //         // );
    //         new_vec.push(node);
    //     }
    //     // } else {
    //     //     new_vec.push(node);
    //     // }
    // }

    // let mut new_vec = nodes.clone();
    // let mut new_vec = Vec::new();
    // let test = nodes.clone();
    // // new_vec.sort_by(|a, b, goal| get_distance(a.position, goal) < get_distance(a.position, goal));
    // test.sort_unstable();

    // new_vec.split_at(MAX_NODES).0.iter().collect()

    if nodes.len() < MAX_NODES {
        return nodes;
    }

    let mut new_vec = Vec::new();
    let mut test = nodes.clone();
    // new_vec.sort_by(|a, b, goal| get_distance(a.position, goal) < get_distance(a.position, goal));
    test.sort_unstable();
    test.reverse();

    println!(
        "Closest is {} away.",
        get_distance(test[0].position, test[0].goal)
    );

    let mut i = 0usize;
    for node in test {
        i += 1;
        new_vec.push(node);
        if i >= MAX_NODES {
            break;
        }
    }
    new_vec
}

fn bfs_depth_flat_with_heuristic(node: BfsNode, board: &mut Board) -> usize {
    let mut depth = 0usize;
    let mut nodes = vec![node];
    loop {
        print!("Depth: {}", depth);
        println!("\tNodes: {}", nodes.len());
        let mut nodes_to_try: Vec<BfsNode> = Vec::new();
        for node in nodes {
            // board.explored[node.position.1][node.position.0] = true;
            let current_height = board.map[node.position.1][node.position.0];
            let mut previous = node.previous;
            previous.push(node.position);
            if node.position == board.ending_pos {
                return depth;
            }
            if node.position.0 > 0 {
                let sample_position = (node.position.0 - 1, node.position.1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if !previous.contains(&sample_position) {
                        if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                            nodes_to_try.push(BfsNode::new(
                                sample_position,
                                previous.clone(),
                                board.ending_pos,
                                board.map[sample_position.1][sample_position.0],
                            ));
                            board.explored[sample_position.1][sample_position.0] = true;
                        }
                    }
                }
            }
            if node.position.0 < board.width - 1 {
                let sample_position = (node.position.0 + 1, node.position.1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if !previous.contains(&sample_position) {
                        if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                            nodes_to_try.push(BfsNode::new(
                                sample_position,
                                previous.clone(),
                                board.ending_pos,
                                board.map[sample_position.1][sample_position.0],
                            ));
                            board.explored[sample_position.1][sample_position.0] = true;
                        }
                    }
                }
            }
            if node.position.1 > 0 {
                let sample_position = (node.position.0, node.position.1 - 1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if !previous.contains(&sample_position) {
                        if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                            nodes_to_try.push(BfsNode::new(
                                sample_position,
                                previous.clone(),
                                board.ending_pos,
                                board.map[sample_position.1][sample_position.0],
                            ));
                            board.explored[sample_position.1][sample_position.0] = true;
                        }
                    }
                }
            }
            if node.position.1 < board.height - 1 {
                let sample_position = (node.position.0, node.position.1 + 1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if !previous.contains(&sample_position) {
                        if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                            nodes_to_try.push(BfsNode::new(
                                sample_position,
                                previous.clone(),
                                board.ending_pos,
                                board.map[sample_position.1][sample_position.0],
                            ));
                            board.explored[sample_position.1][sample_position.0] = true;
                        }
                    }
                }
            }
        }
        // if depth > 18 {
        nodes = heuristic(nodes_to_try, board.ending_pos);
        if nodes.len() < 1 {
            panic!("Wtf");
        }
        // } else {
        //     nodes = nodes_to_try;
        // }
        depth += 1;
    }
}

pub fn d12s1(submit: bool) {
    let mut board = input();

    let root = BfsNode::new(board.starting_pos, Vec::new(), board.ending_pos, 0);
    // let tree = vec![root];

    let depth = bfs_depth_flat_with_heuristic(root, &mut board);

    final_answer(depth, submit, DAY, 1);
}

pub fn d12s2(submit: bool) {
    let input = input();
    // final_answer(input.len(), submit, DAY, 2);
}
