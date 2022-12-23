use std::collections::VecDeque;

use indexmap::IndexMap;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 23;

enum MovementDirections {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct ElfDrone {
    x: i64,
    y: i64,
    // direction_proposed: Option<MovementDirections>,
    proposed_location: Option<(i64, i64)>,
}

impl ElfDrone {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            // direction_proposed: None,
            proposed_location: None,
        }
    }

    fn propose_movement(
        &mut self,
        considerations: &VecDeque<MovementDirections>,
        elf_drones: &Vec<ElfDrone>,
        claim_counts: &mut IndexMap<(i64, i64), usize>,
    ) {
        let mut has_north_neighbor = false;
        let mut has_south_neighbor = false;
        let mut has_east_neighbor = false;
        let mut has_west_neighbor = false;
        for elf_drone in elf_drones {
            match elf_drone.y - self.y {
                -1 => match elf_drone.x - self.x {
                    -1 => {
                        has_north_neighbor = true;
                        has_west_neighbor = true;
                    }
                    0 => {
                        has_north_neighbor = true;
                    }
                    1 => {
                        has_north_neighbor = true;
                        has_east_neighbor = true;
                    }
                    _ => {
                        continue;
                    }
                },
                0 => match elf_drone.x - self.x {
                    -1 => {
                        has_west_neighbor = true;
                    }
                    0 => {
                        // you are me
                        continue;
                    }
                    1 => {
                        has_east_neighbor = true;
                    }
                    _ => {
                        continue;
                    }
                },
                1 => match elf_drone.x - self.x {
                    -1 => {
                        has_south_neighbor = true;
                        has_west_neighbor = true;
                    }
                    0 => {
                        has_south_neighbor = true;
                    }
                    1 => {
                        has_south_neighbor = true;
                        has_east_neighbor = true;
                    }
                    _ => {
                        continue;
                    }
                },
                _ => {
                    continue;
                }
            }
        }
        if has_north_neighbor == false
            && has_south_neighbor == false
            && has_east_neighbor == false
            && has_west_neighbor == false
        {
            return;
        }
        for consideration in considerations.iter() {
            match consideration {
                MovementDirections::North => {
                    if has_north_neighbor == false {
                        let my_proposal = (self.x, self.y - 1);
                        if claim_counts.contains_key(&my_proposal) {
                            *claim_counts.get_mut(&my_proposal).unwrap() += 1;
                        } else {
                            claim_counts.insert(my_proposal, 1);
                        }
                        self.proposed_location = Some(my_proposal);
                        return;
                    }
                }
                MovementDirections::South => {
                    if has_south_neighbor == false {
                        let my_proposal = (self.x, self.y + 1);
                        if claim_counts.contains_key(&my_proposal) {
                            *claim_counts.get_mut(&my_proposal).unwrap() += 1;
                        } else {
                            claim_counts.insert(my_proposal, 1);
                        }
                        self.proposed_location = Some(my_proposal);
                        return;
                    }
                }
                MovementDirections::East => {
                    if has_east_neighbor == false {
                        let my_proposal = (self.x + 1, self.y);
                        if claim_counts.contains_key(&my_proposal) {
                            *claim_counts.get_mut(&my_proposal).unwrap() += 1;
                        } else {
                            claim_counts.insert(my_proposal, 1);
                        }
                        self.proposed_location = Some(my_proposal);
                        return;
                    }
                }
                MovementDirections::West => {
                    if has_west_neighbor == false {
                        let my_proposal = (self.x - 1, self.y);
                        if claim_counts.contains_key(&my_proposal) {
                            *claim_counts.get_mut(&my_proposal).unwrap() += 1;
                        } else {
                            claim_counts.insert(my_proposal, 1);
                        }
                        self.proposed_location = Some(my_proposal);
                        return;
                    }
                }
            }
        }
    }

    fn perform_movement(&mut self, claim_counts: &mut IndexMap<(i64, i64), usize>) -> bool {
        if self.proposed_location.is_some()
            && *claim_counts.get(&self.proposed_location.unwrap()).unwrap() == 1
        {
            self.x = self.proposed_location.unwrap().0;
            self.y = self.proposed_location.unwrap().1;
            self.proposed_location = None;
            return true;
        }
        self.proposed_location = None;
        false
    }
}

fn input() -> Vec<ElfDrone> {
    let raw = input_raw(DAY);
    let lines: Vec<String> = raw.split("\n").map(|item| item.to_owned()).collect();
    let mut elves: Vec<ElfDrone> = Vec::new();
    for y in 0..lines.len() {
        let line = &lines[y];
        let line_chars: Vec<char> = line.chars().collect();
        for x in 0..line_chars.len() {
            match &line_chars[x] {
                '#' => {
                    elves.push(ElfDrone::new(x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    elves
}

fn move_elves(elves: &mut Vec<ElfDrone>, considerations: &VecDeque<MovementDirections>) -> usize {
    let mut claim_counts: IndexMap<(i64, i64), usize> = IndexMap::new();
    let elf_check = elves.clone();
    for elf_drone in elves.iter_mut() {
        elf_drone.propose_movement(considerations, &elf_check, &mut claim_counts);
    }
    let mut moves_made = 0usize;
    for elf_drone in elves.iter_mut() {
        if elf_drone.perform_movement(&mut claim_counts) {
            moves_made += 1;
        }
    }

    moves_made
}

pub fn d23s1(submit: bool) {
    let mut elves = input();
    let mut considerations = VecDeque::from([
        MovementDirections::North,
        MovementDirections::South,
        MovementDirections::West,
        MovementDirections::East,
    ]);
    for _round in 0..10 {
        // println!("DEBUG:\n{:?}", elves);

        print_elves(&elves, -5, 15, -5, 15);
        println!("\n\n");
        move_elves(&mut elves, &considerations);
        let consideration = considerations.pop_front().unwrap();
        considerations.push_back(consideration);
    }

    println!("\nFINAL:\n{:?}", elves);

    let mut min_x = elves[0].x;
    let mut max_x = elves[0].x;
    let mut min_y = elves[0].y;
    let mut max_y = elves[0].y;
    for elf_drone in elves.iter() {
        if elf_drone.x < min_x {
            min_x = elf_drone.x;
        } else if elf_drone.x > max_x {
            max_x = elf_drone.x;
        }
        if elf_drone.y < min_y {
            min_y = elf_drone.y;
        } else if elf_drone.y > max_y {
            max_y = elf_drone.y;
        }
    }
    print_elves(&elves, -5, 15, -5, 15);
    let answer = ((max_x - min_x + 1) * (max_y - min_y + 1)) - (elves.len() as i64);
    final_answer(answer, submit, DAY, 1);
}

fn print_elves(elves: &Vec<ElfDrone>, min_y: i64, max_y: i64, min_x: i64, max_x: i64) {
    for y in min_y..=max_y {
        'x: for x in min_x..=max_x {
            for elf_drone in elves.iter() {
                if elf_drone.x == x && elf_drone.y == y {
                    print!("#");
                    continue 'x;
                }
            }
            print!(".");
        }
        println!();
    }
}

pub fn d23s2(submit: bool) {
    let mut elves = input();
    let mut considerations = VecDeque::from([
        MovementDirections::North,
        MovementDirections::South,
        MovementDirections::West,
        MovementDirections::East,
    ]);
    let mut round = 0;
    loop {
        round += 1;
        // println!("DEBUG:\n{:?}", elves);

        print_elves(&elves, -5, 15, -5, 15);
        println!("\n\n");
        if move_elves(&mut elves, &considerations) == 0 {
            break;
        }
        let consideration = considerations.pop_front().unwrap();
        considerations.push_back(consideration);
    }
    final_answer(round, submit, DAY, 2);
}
