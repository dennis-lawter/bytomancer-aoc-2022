use std::collections::HashSet;
use std::f32::consts::E;
use std::str::FromStr;

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

pub fn d9s1(submit: bool) {
    let input = input();
    let mut head = Coord::new(0, 0);
    let mut tail = Coord::new(0, 0);
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    for command in input {
        for _ in 0..command.steps {
            head.take_step(command.direction.clone());
            tail.follow(&head);

            tail_positions.insert((tail.x, tail.y));
        }
    }
    println!("Head: {} {}", head.x, head.y);
    println!("Tail: {} {}", tail.x, tail.y);

    final_answer(tail_positions.len(), submit, DAY, 1);
}

pub fn d9s2(submit: bool) {
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
        }
    }
    println!("Head: {} {}", nodes[0].x, nodes[0].y);
    println!("Tail: {} {}", nodes[9].x, nodes[9].y);

    final_answer(tail_positions.len(), submit, DAY, 2);
}
