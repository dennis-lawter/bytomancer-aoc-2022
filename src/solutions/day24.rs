use std::collections::HashMap;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 24;

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

struct Blizzard {
    pos: (usize, usize),
    dir: Direction,
}

struct Game {
    blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
}
impl Game {
    fn free_from_blizzards(&self, pos: &(usize, usize)) -> bool {
        for bliz in self.blizzards.iter() {
            if bliz.pos.0 == pos.0 && bliz.pos.1 == pos.1 {
                return false;
            }
        }

        true
    }
    fn free_from_walls(&self, pos: &(usize, usize)) -> bool {
        if self.goal.0 == pos.0 && self.goal.1 == pos.1 {
            return true;
        }
        pos.0 != 0 && pos.1 != 0 && pos.0 != self.width - 1 && pos.1 != self.height - 1
    }
    fn advance_blizzards(&mut self) {
        for bliz in self.blizzards.iter_mut() {
            match bliz.dir {
                Direction::North => {
                    bliz.pos.1 -= 1;
                    if bliz.pos.1 == 0 {
                        bliz.pos.1 = self.height - 2;
                    }
                }
                Direction::South => {
                    bliz.pos.1 += 1;
                    if bliz.pos.1 == self.height - 1 {
                        bliz.pos.1 = 1;
                    }
                }
                Direction::East => {
                    bliz.pos.0 += 1;
                    if bliz.pos.0 == self.width - 1 {
                        bliz.pos.0 = 1;
                    }
                }
                Direction::West => {
                    bliz.pos.0 -= 1;
                    if bliz.pos.0 == 0 {
                        bliz.pos.0 = self.width - 2;
                    }
                }
                Direction::None => {}
            }
        }
    }

    // fn print_bliz_map(&self) {
    //     for y in 1..self.height - 1 {
    //         for x in 1..self.width - 1 {
    //             let mut found = false;
    //             'x: for bliz in self.blizzards.iter() {
    //                 if bliz.pos.0 == x && bliz.pos.1 == y {
    //                     match bliz.dir {
    //                         Direction::North => {
    //                             print!("^");
    //                             found = true;
    //                             break 'x;
    //                         }
    //                         Direction::South => {
    //                             print!("v");
    //                             found = true;
    //                             break 'x;
    //                         }
    //                         Direction::East => {
    //                             print!(">");
    //                             found = true;
    //                             break 'x;
    //                         }
    //                         Direction::West => {
    //                             print!("<");
    //                             found = true;
    //                             break 'x;
    //                         }
    //                         Direction::None => {}
    //                     }
    //                 }
    //             }
    //             if !found {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // }
}

fn input() -> Game {
    let raw = input_raw(DAY);
    let lines: Vec<String> = raw.split("\n").map(|item| item.to_owned()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut start = (0usize, 0usize);
    let top_chars: Vec<char> = lines[0].chars().collect();
    for x in 0..top_chars.len() {
        let top_char = top_chars[x];
        if top_char != '#' {
            start.0 = x;
            break;
        }
    }
    let mut goal = (0usize, height - 1);
    let bottom_chars: Vec<char> = lines[height - 1].chars().collect();
    for x in 0..bottom_chars.len() {
        let bottom_char = bottom_chars[x];
        if bottom_char != '#' {
            goal.0 = x;
            break;
        }
    }

    let mut blizzards: Vec<Blizzard> = Vec::new();
    for y in 1..height - 1 {
        let line_chars: Vec<char> = lines[y].chars().collect();
        for x in 1..width - 1 {
            match line_chars[x] {
                '^' => {
                    let blizzard = Blizzard {
                        pos: (x, y),
                        dir: Direction::North,
                    };
                    blizzards.push(blizzard);
                }
                'v' => {
                    let blizzard = Blizzard {
                        pos: (x, y),
                        dir: Direction::South,
                    };
                    blizzards.push(blizzard);
                }
                '>' => {
                    let blizzard = Blizzard {
                        pos: (x, y),
                        dir: Direction::East,
                    };
                    blizzards.push(blizzard);
                }
                '<' => {
                    let blizzard = Blizzard {
                        pos: (x, y),
                        dir: Direction::West,
                    };
                    blizzards.push(blizzard);
                }
                _ => {}
            }
        }
    }

    Game {
        blizzards,
        width,
        height,
        start,
        goal,
    }
}

#[derive(Clone, Debug)]
struct Explorer {
    pos: (usize, usize),
    history: Vec<Direction>,
}

fn get_shortest_actions(game: &mut Game, explorer: &Explorer) -> Explorer {
    // let mut actions = 0usize;
    let mut fringe: Vec<Explorer> = Vec::from([explorer.clone()]);
    let winner: Explorer;
    'search: loop {
        // std::thread::sleep(core::time::Duration::from_millis(1000));
        game.advance_blizzards();
        // actions += 1;
        // println!("Actions: {}", actions);
        // println!("Fringe size: {}", fringe.len());
        // game.print_bliz_map();
        let mut to_check: HashMap<(usize, usize), Explorer> = HashMap::new();
        for item in fringe {
            to_check.insert(item.pos.clone(), item);
        }
        fringe = Vec::new();
        for (_pos, check) in to_check {
            if check.pos.0 == game.goal.0 && check.pos.1 == game.goal.1 {
                winner = check;
                break 'search;
            }

            if check.pos.1 > 0 {
                let mut north_check = Explorer {
                    pos: (check.pos.0, check.pos.1 - 1),
                    history: check.history.clone(),
                };
                north_check.history.push(Direction::North);
                if game.free_from_blizzards(&north_check.pos)
                    && game.free_from_walls(&north_check.pos)
                {
                    fringe.push(north_check);
                    // print!("N");
                }
            }

            let mut east_check = Explorer {
                pos: (check.pos.0 + 1, check.pos.1),
                history: check.history.clone(),
            };
            east_check.history.push(Direction::East);
            if game.free_from_blizzards(&east_check.pos) && game.free_from_walls(&east_check.pos) {
                fringe.push(east_check);
                // print!("E");
            }

            let mut west_check = Explorer {
                pos: (check.pos.0 - 1, check.pos.1),
                history: check.history.clone(),
            };
            west_check.history.push(Direction::West);
            if game.free_from_blizzards(&west_check.pos) && game.free_from_walls(&west_check.pos) {
                fringe.push(west_check);
                // print!("W");
            }

            if game.free_from_blizzards(&check.pos) {
                let mut pause_check = Explorer {
                    pos: check.pos,
                    history: check.history.clone(),
                };
                pause_check.history.push(Direction::None);
                fringe.push(pause_check);

                // print!("-");
            }

            if check.pos.1 < game.height {
                let mut south_check = Explorer {
                    pos: (check.pos.0, check.pos.1 + 1),
                    history: check.history.clone(),
                };
                south_check.history.push(Direction::South);
                if game.free_from_blizzards(&south_check.pos)
                    && game.free_from_walls(&south_check.pos)
                {
                    fringe.push(south_check);
                    // print!("S");
                }
            }
        }
        // println!("\n");
    }

    // println!("PATH TO VICTORY:\n{:?}", winner);

    winner
}

pub fn d24s1(submit: bool) {
    let mut game = input();
    let explorer = Explorer {
        pos: game.start,
        history: Vec::new(),
    };
    let winner = get_shortest_actions(&mut game, &explorer);
    let answer = winner.history.len();
    final_answer(answer, submit, DAY, 1);
}

pub fn d24s2(submit: bool) {
    let mut game = input();
    let just_starting = Explorer {
        pos: game.start,
        history: Vec::new(),
    };

    let first_time_to_goal = get_shortest_actions(&mut game, &just_starting);
    let old_start = game.start;
    let old_goal = game.goal;
    game.start = old_goal;
    game.goal = old_start;
    println!("Answer 1: {}", first_time_to_goal.history.len());

    let now_with_snacks = get_shortest_actions(&mut game, &first_time_to_goal);
    let old_start = game.start;
    let old_goal = game.goal;
    game.start = old_goal;
    game.goal = old_start;
    println!("Answer 2: {}", now_with_snacks.history.len());

    let winner = get_shortest_actions(&mut game, &now_with_snacks);

    let answer = winner.history.len();
    // need to add 2 because the history doesn't record landing at the goal
    final_answer(answer + 2, submit, DAY, 2);
}
