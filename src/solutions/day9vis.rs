use core::time;
use std::collections::HashSet;
use std::io::stdout;
use std::io::Write;
use std::str::FromStr;
use std::thread;

use colored::Colorize;
use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 9;

struct Command {
    direction: Direction,
    steps: usize,
}
impl Command {
    pub fn new(direction: Direction, steps: usize) -> Self {
        Self { direction, steps }
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Coord {
    x: i64,
    y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn take_step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    pub fn follow(&mut self, leader: &Coord) {
        let x_distance = (self.x - leader.x).abs();
        let y_distance = (self.y - leader.y).abs();
        if x_distance + y_distance > 2 {
            if leader.x > self.x {
                self.take_step(Direction::Right);
            } else {
                self.take_step(Direction::Left);
            }

            if leader.y > self.y {
                self.take_step(Direction::Down);
            } else {
                self.take_step(Direction::Up);
            }
        } else if x_distance > 1 {
            if leader.x > self.x {
                self.take_step(Direction::Right);
            } else {
                self.take_step(Direction::Left);
            }
        } else if y_distance > 1 {
            if leader.y > self.y {
                self.take_step(Direction::Down);
            } else {
                self.take_step(Direction::Up);
            }
        }
    }
}

fn input() -> Vec<Command> {
    let raw = input_raw(DAY);
    let lines: Vec<&str> = raw.split("\n").collect();
    let mut results: Vec<Command> = Vec::with_capacity(lines.len());
    for line in lines {
        let mut line_split = line.split(" ");
        let (dir, steps) = (
            Direction::from_str(line_split.next().unwrap()).unwrap(),
            str::parse::<usize>(line_split.next().unwrap()).unwrap(),
        );
        results.push(Command::new(dir, steps));
    }

    results
}

pub fn d9s1vis(submit: bool) {
    let input = input();
    let mut head = Coord::new(0, 0);
    let mut tail = Coord::new(0, 0);
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    for command in input {
        for _ in 0..command.steps {
            head.take_step(command.direction.clone());
            tail.follow(&head);

            let _inserted_new = tail_positions.insert((tail.x, tail.y));
            {
                let window_size = crossterm::terminal::size().unwrap();
                let x_min = head.x - ((window_size.0 - 0) / 4) as i64;
                let y_min = head.y - ((window_size.1 - 0) / 2) as i64;
                let x_max = head.x + ((window_size.0 - 0) / 4) as i64;
                let y_max = head.y + ((window_size.1 - 0) / 2) as i64;

                let mut stdout = stdout();
                stdout
                    .execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap();
                cursor::MoveTo(0, 0);

                for y in y_min..y_max {
                    for x in x_min..x_max {
                        let test_pos = (x, y);
                        let print_character: String;
                        if test_pos == (head.x, head.y) {
                            print_character = "🤴".to_owned();
                        } else if test_pos == (tail.x, tail.y) {
                            print_character = "🧝".to_owned();
                        } else if tail_positions.contains(&test_pos) {
                            print_character = "👣".to_owned();
                        } else {
                            print_character = "　".to_owned();
                        }
                        if x == 0 || y == 0 {
                            stdout
                                .queue(style::Print(print_character.on_white()))
                                .unwrap();
                        } else {
                            stdout.queue(style::Print(print_character)).unwrap();
                        }
                    }
                    stdout.queue(style::Print("\n")).unwrap();
                }

                stdout.flush().unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }

    final_answer(tail_positions.len(), submit, DAY, 1);
}

pub fn d9s2vis(submit: bool) {
    let input = input();
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    let mut nodes: Vec<Coord> = vec![Coord::new(0, 0); 10];
    for command in &input {
        for _ in 0..command.steps {
            nodes[0].take_step(command.direction.clone());
            for i in 1..nodes.len() {
                let prev = nodes[i - 1].clone();
                nodes[i].follow(&prev);
            }

            tail_positions.insert((nodes[9].x, nodes[9].y));

            {
                let window_size = crossterm::terminal::size().unwrap();
                let x_min = nodes[0].x - ((window_size.0 - 0) / 4) as i64;
                let y_min = nodes[0].y - ((window_size.1 - 0) / 2) as i64;
                let x_max = nodes[0].x + ((window_size.0 - 0) / 4) as i64;
                let y_max = nodes[0].y + ((window_size.1 - 0) / 2) as i64;

                let mut stdout = stdout();
                stdout
                    .execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap();
                cursor::MoveTo(0, 0);

                for y in y_min..y_max {
                    for x in x_min..x_max {
                        let test_pos = (x, y);
                        let print_character: String;
                        if test_pos == (nodes[0].x, nodes[0].y) {
                            print_character = "🤴".to_owned();
                        } else if test_pos == (nodes[9].x, nodes[9].y) {
                            print_character = "🧝".to_owned();
                        } else if test_pos == (nodes[1].x, nodes[1].y)
                            || test_pos == (nodes[2].x, nodes[2].y)
                            || test_pos == (nodes[3].x, nodes[3].y)
                            || test_pos == (nodes[4].x, nodes[4].y)
                            || test_pos == (nodes[5].x, nodes[5].y)
                            || test_pos == (nodes[6].x, nodes[6].y)
                            || test_pos == (nodes[7].x, nodes[7].y)
                            || test_pos == (nodes[8].x, nodes[8].y)
                        {
                            print_character = "🪢".to_owned();
                        } else if tail_positions.contains(&test_pos) {
                            print_character = "👣".to_owned();
                        } else {
                            print_character = "　".to_owned();
                        }
                        if x == 0 || y == 0 {
                            stdout
                                .queue(style::Print(print_character.on_white()))
                                .unwrap();
                        } else {
                            stdout.queue(style::Print(print_character)).unwrap();
                        }
                    }
                    stdout.queue(style::Print("\n")).unwrap();
                }
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }

    final_answer(tail_positions.len(), submit, DAY, 2);
}
