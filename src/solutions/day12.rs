use super::final_answer;
use super::input_raw;

const DAY: u8 = 12;

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
        let map_row: Vec<u8> = lines[i]
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

fn bfs_depth_flat_with_heuristic(root_nodes: Vec<(usize, usize)>, board: &mut Board) -> usize {
    let mut depth = 0usize;
    let mut nodes = root_nodes.clone();
    loop {
        print!("Depth: {}", depth);
        println!("\tNodes: {}", nodes.len());
        let mut nodes_to_try: Vec<(usize, usize)> = Vec::new();
        for node in nodes {
            board.explored[node.1][node.0] = true;
            let current_height = board.map[node.1][node.0];
            if node == board.ending_pos {
                return depth;
            }
            if node.0 > 0 {
                let sample_node = (node.0 - 1, node.1);
                if board.explored[sample_node.1][sample_node.0] == false {
                    if board.map[sample_node.1][sample_node.0] <= current_height + 1 {
                        nodes_to_try.push(sample_node);
                        board.explored[sample_node.1][sample_node.0] = true;
                    }
                }
            }
            if node.0 < board.width - 1 {
                let sample_node = (node.0 + 1, node.1);
                if board.explored[sample_node.1][sample_node.0] == false {
                    if board.map[sample_node.1][sample_node.0] <= current_height + 1 {
                        nodes_to_try.push(sample_node);
                        board.explored[sample_node.1][sample_node.0] = true;
                    }
                }
            }
            if node.1 > 0 {
                let sample_node = (node.0, node.1 - 1);
                if board.explored[sample_node.1][sample_node.0] == false {
                    if board.map[sample_node.1][sample_node.0] <= current_height + 1 {
                        nodes_to_try.push(sample_node);
                        board.explored[sample_node.1][sample_node.0] = true;
                    }
                }
            }
            if node.1 < board.height - 1 {
                let sample_node = (node.0, node.1 + 1);
                if board.explored[sample_node.1][sample_node.0] == false {
                    if board.map[sample_node.1][sample_node.0] <= current_height + 1 {
                        nodes_to_try.push(sample_node);
                        board.explored[sample_node.1][sample_node.0] = true;
                    }
                }
            }
        }
        nodes = nodes_to_try;
        if nodes.len() < 1 {
            panic!("Wtf");
        }
        depth += 1;
    }
}

pub fn d12s1(submit: bool) {
    let mut board = input();

    let tree = vec![board.starting_pos];

    let depth = bfs_depth_flat_with_heuristic(tree, &mut board);

    final_answer(depth, submit, DAY, 1);
}

pub fn d12s2(submit: bool) {
    let mut board = input();

    let mut tree: Vec<(usize, usize)> = Vec::new();
    for y in 0..board.height {
        for x in 0..board.width {
            if board.map[y][x] == 1u8 {
                tree.push((x, y));
            }
        }
    }

    let depth = bfs_depth_flat_with_heuristic(tree, &mut board);

    final_answer(depth, submit, DAY, 2);
}
