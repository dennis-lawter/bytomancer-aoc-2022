use std::collections::HashSet;
use std::collections::VecDeque;

use colored::Colorize;
use crossterm::ExecutableCommand;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 14;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Point(u32, u32);

fn input() -> Vec<Vec<Point>> {
    let raw = input_raw(DAY);
    let mut result = Vec::with_capacity(raw.len());

    let lines = raw.split("\n");
    for line in lines {
        let coord_strs: Vec<&str> = line.split(" -> ").collect();
        let mut result_line: Vec<Point> = Vec::with_capacity(coord_strs.len());
        for coord_str in coord_strs {
            let mut coord_split = coord_str.split(",");
            let x = str::parse::<u32>(coord_split.next().unwrap()).unwrap();
            let y = str::parse::<u32>(coord_split.next().unwrap()).unwrap();
            result_line.push(Point(x, y));
        }
        result.push(result_line);
    }

    result
}

pub fn d14s1vis(submit: bool) {
    crossterm::terminal::enable_raw_mode().expect("Raw mode not supported");
    std::io::stdout().execute(crossterm::cursor::Hide).unwrap();
    let _clean_up = CleanUp;
    let rocks = input();

    const DROP_POINT: Point = Point(500, 0);

    let mut y_max = 0u32;

    for rock in &rocks {
        for point in rock {
            if point.1 > y_max {
                y_max = point.1;
            }
        }
    }
    y_max += 1;

    for _ in 0..=y_max + 1 {
        print!("\r\n");
    }

    let mut stone_locations: HashSet<Point> = HashSet::new();
    for rock in &rocks {
        let mut i = 1usize;
        while i < rock.len() {
            let loc_0 = &rock[i - 1];
            let loc_1 = &rock[i];
            if loc_0.0 == loc_1.0 {
                let x = loc_0.0;
                let y_start = if loc_0.1 < loc_1.1 { loc_0.1 } else { loc_1.1 };
                let y_end = if loc_0.1 > loc_1.1 { loc_0.1 } else { loc_1.1 };
                for y in y_start..=y_end {
                    stone_locations.insert(Point(x, y));
                }
            } else if loc_0.1 == loc_1.1 {
                let y = loc_0.1;
                let x_start = if loc_0.0 < loc_1.0 { loc_0.0 } else { loc_1.0 };
                let x_end = if loc_0.0 > loc_1.0 { loc_0.0 } else { loc_1.0 };
                for x in x_start..=x_end {
                    stone_locations.insert(Point(x, y));
                }
            } else {
                panic!("Rocks can go diagonally...?");
            }
            i += 1usize;
        }
    }

    let mut sand_locations: HashSet<Point> = HashSet::with_capacity(30_000);

    let mut falling_grains: VecDeque<Point> = VecDeque::with_capacity(1_000);

    // let mut frame: u64 = 0;

    'falling_sand_game: loop {
        falling_grains.push_back(Point(DROP_POINT.0, DROP_POINT.1));
        for _ in 0..falling_grains.len() {
            let mut falling_grain = falling_grains.pop_front().unwrap();
            let neighbors = [
                Point(falling_grain.0, falling_grain.1 + 1),
                Point(falling_grain.0 - 1, falling_grain.1 + 1),
                Point(falling_grain.0 + 1, falling_grain.1 + 1),
            ];
            let mut fell: bool = false;
            for neighbor in neighbors {
                if !sand_locations.contains(&neighbor) && !stone_locations.contains(&neighbor) {
                    falling_grain.0 = neighbor.0;
                    falling_grain.1 = neighbor.1;
                    fell = true;
                    break;
                }
            }
            // The sand rests
            if falling_grain.1 > y_max {
                break 'falling_sand_game;
            } else if !fell {
                sand_locations.insert(Point(falling_grain.0, falling_grain.1));
            } else {
                falling_grains.push_back(falling_grain);
            }
        }
        display(y_max, &stone_locations, &sand_locations, &falling_grains);
    }

    display(y_max, &stone_locations, &sand_locations, &falling_grains);

    crossterm::terminal::disable_raw_mode().unwrap();

    final_answer(sand_locations.len(), submit, DAY, 1);
}

pub fn d14s2vis(submit: bool) {
    crossterm::terminal::enable_raw_mode().expect("Raw mode not supported");
    std::io::stdout().execute(crossterm::cursor::Hide).unwrap();
    let _clean_up = CleanUp;
    let rocks = input();

    const DROP_POINT: Point = Point(500, 0);

    let mut y_max = 0u32;

    for rock in &rocks {
        for point in rock {
            if point.1 > y_max {
                y_max = point.1;
            }
        }
    }
    y_max += 1;

    for _ in 0..=y_max + 1 {
        print!("\r\n");
    }

    let mut stone_locations: HashSet<Point> = HashSet::new();
    for rock in &rocks {
        let mut i = 1usize;
        while i < rock.len() {
            let loc_0 = &rock[i - 1];
            let loc_1 = &rock[i];
            if loc_0.0 == loc_1.0 {
                let x = loc_0.0;
                let y_start = if loc_0.1 < loc_1.1 { loc_0.1 } else { loc_1.1 };
                let y_end = if loc_0.1 > loc_1.1 { loc_0.1 } else { loc_1.1 };
                for y in y_start..=y_end {
                    stone_locations.insert(Point(x, y));
                }
            } else if loc_0.1 == loc_1.1 {
                let y = loc_0.1;
                let x_start = if loc_0.0 < loc_1.0 { loc_0.0 } else { loc_1.0 };
                let x_end = if loc_0.0 > loc_1.0 { loc_0.0 } else { loc_1.0 };
                for x in x_start..=x_end {
                    stone_locations.insert(Point(x, y));
                }
            } else {
                panic!("Rocks can go diagonally...?");
            }
            i += 1usize;
        }
    }

    let mut sand_locations: HashSet<Point> = HashSet::with_capacity(30_000);

    let mut falling_grains: VecDeque<Point> = VecDeque::with_capacity(1_000);

    let mut frame: u64 = 0;

    'falling_sand_game: loop {
        falling_grains.push_back(Point(DROP_POINT.0, DROP_POINT.1));
        for _ in 0..falling_grains.len() {
            let mut falling_grain = falling_grains.pop_front().unwrap();
            if falling_grain.1 == y_max {
                // simulated floor
                sand_locations.insert(Point(falling_grain.0, falling_grain.1));
                continue;
            }
            let neighbors = [
                Point(falling_grain.0, falling_grain.1 + 1),
                Point(falling_grain.0 - 1, falling_grain.1 + 1),
                Point(falling_grain.0 + 1, falling_grain.1 + 1),
            ];
            let mut fell: bool = false;
            for neighbor in neighbors {
                if !sand_locations.contains(&neighbor) && !stone_locations.contains(&neighbor) {
                    falling_grain.0 = neighbor.0;
                    falling_grain.1 = neighbor.1;
                    fell = true;
                    break;
                }
            }
            // The sand rests
            if falling_grain.0 == DROP_POINT.0 && falling_grain.1 == DROP_POINT.1 {
                sand_locations.insert(Point(falling_grain.0, falling_grain.1));
                break 'falling_sand_game;
            } else if !fell {
                sand_locations.insert(Point(falling_grain.0, falling_grain.1));
            } else {
                falling_grains.push_back(falling_grain);
            }
        }
        if frame % 20 == 0 {
            display(y_max, &stone_locations, &sand_locations, &falling_grains);
        }
        frame += 1;
    }

    display(y_max, &stone_locations, &sand_locations, &falling_grains);

    crossterm::terminal::disable_raw_mode().unwrap();

    final_answer(sand_locations.len(), submit, DAY, 2);
}

fn display(
    y_max: u32,
    stone_locations: &HashSet<Point>,
    sand_locations: &HashSet<Point>,
    falling_grains: &VecDeque<Point>,
) {
    std::io::stdout()
        .execute(crossterm::cursor::MoveToPreviousLine((y_max + 3) as u16))
        .unwrap();
    let terminal_size = crossterm::terminal::size().unwrap();
    let terminal_width = terminal_size.0 - 8;
    let x_min = 500 - terminal_width / 4;
    let x_max = 500 + terminal_width / 4;
    print!("\r\n");
    for y in 0..=y_max {
        for x in x_min..x_max {
            let test_point = Point(x as u32, y);
            if stone_locations.contains(&test_point) {
                print!("{}", String::from("　").on_white());
            } else if sand_locations.contains(&test_point) {
                print!("{}", String::from("　").on_yellow());
            } else if falling_grains.contains(&test_point) {
                print!("{}", String::from("　").on_red());
            } else {
                print!("　");
            }
        }
        print!("\r\n");
    }
    for _ in x_min..x_max {
        print!("{}", String::from("　").on_white());
    }
    print!("\r\n");

    std::thread::sleep(std::time::Duration::from_millis(15));
}
