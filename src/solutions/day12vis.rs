use colored::Colorize;
use core::time;
use std::io::stdout;
use std::thread;

use crossterm::cursor::MoveToPreviousLine;
use crossterm::ExecutableCommand;

// use super::final_answer;
use super::input_raw;

const DAY: u8 = 12;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
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
impl Board {
    pub fn get_cell(&self, x: usize, y: usize, starting_height: u8) -> String {
        if self.ending_pos == (x, y) {
            String::from("🚩")
        } else if self.map[y][x] == starting_height {
            String::from("🏁")
        } else {
            format!("{} ", (self.map[y][x] + ('a' as u8) - 1) as char)
        }
    }
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

#[derive(Clone)]
struct BfsNode {
    position: (usize, usize),
    previous: Vec<(usize, usize)>,
}
impl BfsNode {
    fn new(position: (usize, usize), previous: Vec<(usize, usize)>) -> Self {
        Self { position, previous }
    }
    fn x(&self) -> usize {
        self.position.0
    }
    fn y(&self) -> usize {
        self.position.1
    }
}

fn nodes_contains_position(nodes: &Vec<BfsNode>, position: &(usize, usize)) -> bool {
    for node in nodes {
        if node.position == *position {
            return true;
        }
    }

    false
}

fn bfs(root_nodes: Vec<BfsNode>, board: &mut Board, start_height: u8) -> BfsNode {
    let mut depth = 0usize;
    let mut nodes = root_nodes.clone();
    loop {
        let mut nodes_to_try: Vec<BfsNode> = Vec::new();
        for node in nodes {
            board.explored[node.y()][node.x()] = true;
            let current_height = board.map[node.y()][node.x()];
            if node.position == board.ending_pos {
                return node;
            }
            if (&node).x() > 0 {
                let sample_position = (node.x() - 1, node.y());
                if board.explored[sample_position.1][sample_position.0] == false {
                    if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                        let mut previous_clone = (&node).previous.clone();
                        previous_clone.push(node.position);
                        let sample_node = BfsNode::new(sample_position, previous_clone);
                        board.explored[sample_node.y()][sample_node.x()] = true;
                        nodes_to_try.push(sample_node);
                    }
                }
            }
            if (&node).x() < board.width - 1 {
                let sample_position = (node.x() + 1, node.y());
                if board.explored[sample_position.1][sample_position.0] == false {
                    if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                        let mut previous_clone = (&node).previous.clone();
                        previous_clone.push(node.position);
                        let sample_node = BfsNode::new(sample_position, previous_clone);
                        board.explored[sample_node.y()][sample_node.x()] = true;
                        nodes_to_try.push(sample_node);
                    }
                }
            }
            if (&node).y() > 0 {
                let sample_position = (node.x(), node.y() - 1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                        let mut previous_clone = (&node).previous.clone();
                        previous_clone.push(node.position);
                        let sample_node = BfsNode::new(sample_position, previous_clone);
                        board.explored[sample_node.y()][sample_node.x()] = true;
                        nodes_to_try.push(sample_node);
                    }
                }
            }
            if (&node).y() < board.height - 1 {
                let sample_position = (node.x(), node.y() + 1);
                if board.explored[sample_position.1][sample_position.0] == false {
                    if board.map[sample_position.1][sample_position.0] <= current_height + 1 {
                        let mut previous_clone = (&node).previous.clone();
                        previous_clone.push(node.position);
                        let sample_node = BfsNode::new(sample_position, previous_clone);
                        board.explored[sample_node.y()][sample_node.x()] = true;
                        nodes_to_try.push(sample_node);
                    }
                }
            }
        }
        nodes = nodes_to_try;
        depth += 1;

        stdout()
            .execute(MoveToPreviousLine(board.height as u16))
            .unwrap();

        for y in 0..board.height {
            for x in 0..board.width {
                if nodes_contains_position(&nodes, &(x, y)) {
                    if board.get_cell(x, y, start_height) == "🚩" {
                        print!("🏆");
                    } else {
                        print!("{}", board.get_cell(x, y, start_height).on_blue());
                    }
                } else if board.explored[y][x] {
                    print!("{}", board.get_cell(x, y, start_height).on_red());
                } else {
                    print!("{}", board.get_cell(x, y, start_height));
                }
            }
            print!("\r\n");
        }
        print!("{}", format!("STEPS: {}", depth).bold().on_blue());

        thread::sleep(time::Duration::from_millis(16));
    }
}

pub fn d12s1vis(_submit: bool) {
    crossterm::terminal::enable_raw_mode().expect("Raw mode not supported");
    let _clean_up = CleanUp;

    let mut board = input();

    let start_height = 0u8;

    for y in 0..board.height {
        for x in 0..board.width {
            print!("{}", board.get_cell(x, y, start_height));
        }
        print!("\r\n");
    }
    // print!("\r\n\r\n");

    let tree = vec![BfsNode::new(board.starting_pos, Vec::new())];

    let winner = bfs(tree, &mut board, start_height);

    stdout()
        .execute(MoveToPreviousLine(board.height as u16))
        .unwrap();

    for y in 0..board.height {
        for x in 0..board.width {
            if board.get_cell(x, y, start_height) == "🚩" {
                print!("🏆");
            } else if winner.previous.contains(&(x, y)) {
                print!(
                    "{}",
                    board.get_cell(x, y, start_height).bold().black().on_white()
                );
            } else if board.explored[y][x] {
                print!("{}", board.get_cell(x, y, start_height).on_red());
            } else {
                print!("{}", board.get_cell(x, y, start_height));
            }
        }
        print!("\r\n");
    }
    print!(
        "{}",
        format!("STEPS: {}", winner.previous.len()).on_blue().bold()
    );

    print!("\r\n");
}

pub fn d12s2vis(_submit: bool) {
    crossterm::terminal::enable_raw_mode().expect("Raw mode not supported");
    let _clean_up = CleanUp;

    let mut board = input();

    let start_height = 1u8;

    for y in 0..board.height {
        for x in 0..board.width {
            print!("{}", board.get_cell(x, y, start_height));
        }
        print!("\r\n");
    }
    // print!("\r\n\r\n");

    let mut tree: Vec<BfsNode> = Vec::new();
    for y in 0..board.height {
        for x in 0..board.width {
            if board.map[y][x] == start_height {
                tree.push(BfsNode::new((x, y), Vec::new()));
            }
        }
    }

    let winner = bfs(tree, &mut board, 1);

    stdout()
        .execute(MoveToPreviousLine(board.height as u16))
        .unwrap();

    for y in 0..board.height {
        for x in 0..board.width {
            if board.get_cell(x, y, start_height) == "🚩" {
                print!("🏆");
            } else if winner.previous.contains(&(x, y)) {
                print!(
                    "{}",
                    board.get_cell(x, y, start_height).bold().black().on_white()
                );
            } else if board.explored[y][x] {
                print!("{}", board.get_cell(x, y, start_height).on_red());
            } else {
                print!("{}", board.get_cell(x, y, start_height));
            }
        }
        print!("\r\n");
    }
    print!(
        "{}",
        format!("STEPS: {}", winner.previous.len()).on_blue().bold()
    );

    print!("\r\n");
}
