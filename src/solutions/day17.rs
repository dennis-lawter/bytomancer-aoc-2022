// use core::time;
// use core::time;
use std::collections::VecDeque;
// use std::thread;

// use indicatif::ProgressBar;
// use indicatif::ProgressStyle;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 17;

fn input() -> Vec<char> {
    input_raw(DAY).chars().into_iter().collect()
}

const SOLID_ROCKS_WINDOW_SIZE: usize = 1_000_000;

// const SOLID_ROCKS_WINDOW_SIZE: usize = 1_000;

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
    fn new(input: &char) -> Self {
        match input {
            '<' => Self::Left,
            '>' => Self::Right,
            invalid_input => panic!("Invalid char: {}", invalid_input),
        }
    }
}

#[derive(Debug)]
struct Board {
    solid_lines: VecDeque<u8>,
    active_lines: Vec<u8>,
    active_lines_offset: usize,
    count_solid_rocks: usize,
    floor_offset: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            solid_lines: vec![0xff].into_iter().collect(), // floor at 0
            active_lines: Vec::with_capacity(10),
            active_lines_offset: 0,
            count_solid_rocks: 0, // floor doesn't count
            floor_offset: 0,
        }
    }
    fn print(&self) {
        let starting_line = self.solid_lines.len() + self.floor_offset + 5;
        let mut i = starting_line;
        let mut futility = 0;
        while i > 0 && futility < 25 {
            let mut scanline = 0u8;
            // if i >= self.active_lines_offset && i - starting_line < self.active_lines.len() {
            if i >= self.active_lines_offset
                && i < self.active_lines_offset + self.active_lines.len()
            {
                scanline |= self.active_lines[i - self.active_lines_offset];
            }
            if i < self.solid_lines.len() + self.floor_offset {
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
            futility += 1;
        }
        println!("└──────────────┘")
    }
    fn insert_falling_rock(&mut self, rock: &Rock) {
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

    fn game_cycle_with_breeze(&mut self, direction: &BreezeDirection) {
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
        // if self.active_lines_offset > self.solid_lines.len() + self.floor_offset - 1 {
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
        // let blank_rows_needed_count = self.active_lines.len();
        // for _ in 0..blank_rows_needed_count {
        //     self.solid_lines.push_back(0);
        // }
        for i in 0..self.active_lines.len() {
            let from_active = self.active_lines.get_mut(i).unwrap();
            match self.solid_lines.get_mut(i + self.active_lines_offset) {
                Some(from_solid) => {
                    *from_solid = *from_solid | *from_active;
                }
                None => {
                    self.solid_lines.push_back(*from_active);
                }
            }
        }
        self.blank_active_lines();
        self.count_solid_rocks += 1;

        while self.solid_lines[self.solid_lines.len() - 1] == 0 {
            self.solid_lines.pop_back();
        }
        while self.solid_lines.len() > SOLID_ROCKS_WINDOW_SIZE * 2 {
            self.solid_lines = self.solid_lines.split_off(SOLID_ROCKS_WINDOW_SIZE);
            // self.solid_lines.pop_front();
            self.floor_offset += SOLID_ROCKS_WINDOW_SIZE;
            // self.active_lines_offset -= 1;
        }
        // while self.solid_lines
        // for self.self.solid_lines
    }

    fn blank_active_lines(&mut self) {
        self.active_lines = Vec::with_capacity(4);
    }
}

pub fn d17s1(submit: bool) {
    let input = input();
    let mut game = Board::new();
    let mut rock_order: Vec<Rock> = Vec::with_capacity(5);
    rock_order.push(Rock::HBar);
    rock_order.push(Rock::Plus);
    rock_order.push(Rock::Corner);
    rock_order.push(Rock::VBar);
    rock_order.push(Rock::Square);
    let mut rock_iter = 0usize;
    let mut i = 0usize;
    while game.count_solid_rocks < 2022 {
        // println!("Solid rocks so far: {}", game.count_solid_rocks);
        // println!("Game debug: {:?}", game);
        if game.does_game_need_new_rock() {
            let rock = &rock_order[rock_iter % 5];
            rock_iter += 1;
            // println!("Inserting {:?}", rock);
            game.insert_falling_rock(rock);

            // game.print();
            // panic!("TEST");
            // thread::sleep(time::Duration::from_millis(1000));
        }
        let breeze = BreezeDirection::new(&input[i]);
        game.game_cycle_with_breeze(&breeze);
        i += 1;
        if i >= input.len() {
            i = 0;
        }

        // game.print();
        // panic!("TEST");
        // thread::sleep(time::Duration::from_millis(1000));
    }
    final_answer(
        game.solid_lines.len() + game.floor_offset - 1,
        submit,
        DAY,
        1,
    );
}

pub fn d17s2(submit: bool) {
    let input = input();
    let mut game = Board::new();
    let mut rock_order: Vec<Rock> = Vec::with_capacity(5);
    rock_order.push(Rock::HBar);
    rock_order.push(Rock::Plus);
    rock_order.push(Rock::Corner);
    rock_order.push(Rock::VBar);
    rock_order.push(Rock::Square);
    let mut rock_iter = 0usize;
    // let bar_style = ProgressStyle::with_template(
    //     "[{elapsed_precise} elapsed] [{eta_precise} remaining] [{percent:.2}%] [{human_pos:>7} M/{human_len:7} M] {msg}\n{bar:80.cyan/blue} ",
    // )
    // .unwrap();
    // let bar = ProgressBar::new(1000000000000).with_style(bar_style);
    // bar.enable_steady_tick(time::Duration::from_millis(1000));
    let mut i = 0usize;
    let mut cycles = 0usize;
    let mut rock_count_for_first_cycle = 0usize;
    let mut height_for_first_cycle = 0usize;
    let mut rock_count_for_second_cycle = 0;
    let mut height_for_second_cycle = 0usize;
    let mut final_answer_increase = 0usize;
    // let mut rock_count_for_third_cycle = 0;
    while game.count_solid_rocks < 1000000000000usize {
        // println!("Solid rocks so far: {}", game.count_solid_rocks);
        // println!("Game debug: {:?}", game);
        if game.does_game_need_new_rock() {
            if i == 0 && rock_iter % 5 == 0 && game.count_solid_rocks > 0 {
                // game.print();
                panic!("Groundhog Day found: {}", game.count_solid_rocks);
            }
            let rock = &rock_order[rock_iter % 5];
            rock_iter += 1;
            // println!("Inserting {:?}", rock);
            game.insert_falling_rock(rock);
            // rock_order.push_back(rock);
            // if game.count_solid_rocks % 1_000_000 == 0 {
            //     bar.inc(1);
            // }

            // game.print();
            // panic!("TEST");
            // thread::sleep(time::Duration::from_millis(1000));
        }
        let breeze = BreezeDirection::new(&input[i]);
        game.game_cycle_with_breeze(&breeze);
        i += 1;
        if i >= input.len() {
            i = 0;
            println!("ROCKS FALLEN: {}", game.count_solid_rocks);
            // game.print();
            if cycles == 0 {
                // store first cycle as a weird offset
                rock_count_for_first_cycle = game.count_solid_rocks;
                height_for_first_cycle = game.solid_lines.len();
            }
            if cycles == 1 {
                rock_count_for_second_cycle = game.count_solid_rocks - rock_count_for_first_cycle;
                height_for_second_cycle = game.solid_lines.len() - height_for_first_cycle;

                while game.count_solid_rocks + rock_count_for_second_cycle < 1000000000000usize {
                    game.count_solid_rocks += rock_count_for_second_cycle;
                    final_answer_increase += height_for_second_cycle;
                }
            }
            // if cycles == 2 {
            //     rock_count_for_third_cycle = game.count_solid_rocks
            //         - rock_count_for_second_cycle
            //         - rock_count_for_first_cycle;
            //     // break 'gameloop;
            // while game.count_solid_rocks + rock_count_for_second_cycle < 1000000000000usize {
            //     game.count_solid_rocks += rock_count_for_second_cycle;
            // }
            // }
            game.print();
            cycles += 1;
        }

        // game.print();
        // panic!("TEST");
        // thread::sleep(time::Duration::from_millis(1000));
    }

    println!(
        "FIRST: {}\tHEIGHT: {}\nSECOND: {}\tHEIGHT: {}\n\n",
        rock_count_for_first_cycle,
        height_for_first_cycle,
        rock_count_for_second_cycle,
        height_for_second_cycle
    );
    game.print();
    // bar.finish();
    final_answer(
        (game.solid_lines.len() + game.floor_offset - 1) + final_answer_increase,
        submit,
        DAY,
        2,
    );
}
