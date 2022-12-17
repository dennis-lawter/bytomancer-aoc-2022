// use core::time;
use std::collections::VecDeque;
// use std::thread;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 17;

fn input() -> Vec<char> {
    input_raw(DAY).chars().into_iter().collect()
}

#[derive(Clone, Debug)]
enum Rock {
    HBar,
    Plus,
    Corner,
    VBar,
    Square,
}

enum BreezeDirection {
    Left,
    Right,
}
impl BreezeDirection {
    fn new(input: char) -> Self {
        match input {
            '<' => Self::Left,
            '>' => Self::Right,
            invalid_input => panic!("Invalid char: {}", invalid_input),
        }
    }
}

#[derive(Debug)]
struct Board {
    solid_lines: Vec<u8>,
    active_lines: Vec<u8>,
    active_lines_offset: usize,
    count_solid_rocks: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            solid_lines: vec![0xff], // floor at 0
            active_lines: Vec::new(),
            active_lines_offset: 0,
            count_solid_rocks: 0, // floor doesn't count
        }
    }
    fn print(&self) {
        let starting_line = self.solid_lines.len() + 5;
        let mut i = starting_line;
        while i > 0 {
            let mut scanline = 0u8;
            // if i >= self.active_lines_offset && i - starting_line < self.active_lines.len() {
            if i >= self.active_lines_offset
                && i < self.active_lines_offset + self.active_lines.len()
            {
                scanline |= self.active_lines[i - self.active_lines_offset];
            }
            if i < self.solid_lines.len() {
                scanline |= self.solid_lines[i];
            }
            print!("│");
            let mut scanline_bit = 0b10000000;
            for _ in 0..7 {
                if scanline & scanline_bit > 0 {
                    // print!("🪨");
                    print!("██");
                } else {
                    // print!("　");
                    print!("  ");
                }
                scanline_bit = scanline_bit >> 1;
            }
            println!("│ {}", i);

            i -= 1;
        }
        println!("└──────────────┘")
    }
    fn insert_falling_rock(&mut self, rock: Rock) {
        self.active_lines_offset = self.solid_lines.len() + 3;
        // for _ in 0..3 {
        //     self.active_lines.push(0);
        // }
        match rock {
            // REMEMBER THESE ARE UPSIDE DOWN DUE TO PUSH ORDER
            Rock::HBar => {
                self.active_lines.push(0b00111100);
            }
            Rock::Plus => {
                self.active_lines.push(0b00010000);
                self.active_lines.push(0b00111000);
                self.active_lines.push(0b00010000);
            }
            Rock::Corner => {
                self.active_lines.push(0b00111000);
                self.active_lines.push(0b00001000);
                self.active_lines.push(0b00001000);
            }
            Rock::VBar => {
                self.active_lines.push(0b00100000);
                self.active_lines.push(0b00100000);
                self.active_lines.push(0b00100000);
                self.active_lines.push(0b00100000);
            }
            Rock::Square => {
                self.active_lines.push(0b00110000);
                self.active_lines.push(0b00110000);
            }
        }
    }

    fn game_cycle_with_breeze(&mut self, direction: BreezeDirection) {
        let mut can_move = true;
        match direction {
            BreezeDirection::Left => {
                for line in self.active_lines.iter() {
                    if *line & 0b10000000 > 0 {
                        can_move = false;
                        // println!("WALL STOPPING; CANNOT MOVE LEFT");
                    }
                }
                if can_move {
                    for line in self.active_lines.iter_mut() {
                        *line = *line << 1;
                    }
                    if self.test_collision() {
                        for line in self.active_lines.iter_mut() {
                            *line = *line >> 1;
                        }
                    }
                }
            }
            BreezeDirection::Right => {
                for line in self.active_lines.iter() {
                    if *line & 0b00000010 > 0 {
                        can_move = false;
                        // println!("WALL STOPPING; CANNOT MOVE RIGHT");
                    }
                }
                if can_move {
                    for line in self.active_lines.iter_mut() {
                        *line = *line >> 1;
                    }
                    if self.test_collision() {
                        for line in self.active_lines.iter_mut() {
                            *line = *line << 1;
                        }
                    }
                }
            }
        }
        // println!("Doing gravity...");
        if self.do_gravity_return_collision() {
            // println!("COLLISION");
            self.blit_active_lines_to_solid();
        }
        // println!("\t\tACTIVE LINE: {}", self.active_lines_offset);
    }

    fn does_game_need_new_rock(&self) -> bool {
        self.active_lines.len() == 0
    }

    fn do_gravity_return_collision(&mut self) -> bool {
        self.active_lines_offset -= 1;
        // if self.active_lines_offset > self.solid_lines.len() - 1 {
        //     return false;
        // }
        if self.test_collision() {
            self.active_lines_offset += 1;
            return true;
        }

        false
    }

    fn test_collision(&mut self) -> bool {
        let mut i = self.active_lines_offset;
        for active_line in self.active_lines.iter() {
            if i > self.solid_lines.len() - 1 {
                continue;
            }
            let test = active_line & self.solid_lines[i];
            if test > 0 {
                return true;
            }
            i += 1;
        }
        false
    }

    fn blit_active_lines_to_solid(&mut self) {
        let blank_rows_needed_count = self.active_lines.len();
        for _ in 0..blank_rows_needed_count {
            self.solid_lines.push(0);
        }
        for i in 0..self.active_lines.len() {
            let from_active = self.active_lines.get_mut(i).unwrap();
            let from_solid = self
                .solid_lines
                .get_mut(i + self.active_lines_offset)
                .unwrap();
            *from_solid = *from_solid | *from_active;
        }
        self.blank_active_lines();
        self.count_solid_rocks += 1;

        while self.solid_lines[self.solid_lines.len() - 1] == 0 {
            self.solid_lines.pop();
        }
    }

    fn blank_active_lines(&mut self) {
        self.active_lines = vec![];
    }
}

pub fn d17s1(submit: bool) {
    let input = input();
    let mut game = Board::new();
    let mut rock_order: VecDeque<Rock> = VecDeque::with_capacity(5);
    rock_order.push_back(Rock::HBar);
    rock_order.push_back(Rock::Plus);
    rock_order.push_back(Rock::Corner);
    rock_order.push_back(Rock::VBar);
    rock_order.push_back(Rock::Square);
    let mut i = 0usize;
    while game.count_solid_rocks < 2022 {
        println!("Solid rocks so far: {}", game.count_solid_rocks);
        // println!("Game debug: {:?}", game);
        if game.does_game_need_new_rock() {
            let rock = rock_order.pop_front().unwrap();
            // println!("Inserting {:?}", rock);
            game.insert_falling_rock(rock.clone());
            rock_order.push_back(rock);

            // game.print();
            // panic!("TEST");
            // thread::sleep(time::Duration::from_millis(1000));
        }
        let breeze = BreezeDirection::new(input[i]);
        game.game_cycle_with_breeze(breeze);
        i += 1;
        if i >= input.len() {
            i = 0;
        }

        // game.print();
        // panic!("TEST");
        // thread::sleep(time::Duration::from_millis(1000));
    }
    final_answer(game.solid_lines.len() - 1, submit, DAY, 1);
}

pub fn d17s2(submit: bool) {
    let input = input();
    final_answer(input.len(), submit, DAY, 2);
}
