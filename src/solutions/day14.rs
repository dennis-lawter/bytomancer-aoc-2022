use std::collections::HashSet;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 14;

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

pub fn d14s1(submit: bool) {
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

    let mut sand_locations: HashSet<Point> = HashSet::new();

    'falling_sand_game: loop {
        let mut new_sand_grain = Point(DROP_POINT.0, DROP_POINT.1);
        'falling_sand_grain: loop {
            if new_sand_grain.1 > y_max {
                // The sand has left the playing field
                break 'falling_sand_game;
            }
            let neighbors = [
                Point(new_sand_grain.0, new_sand_grain.1 + 1),
                Point(new_sand_grain.0 - 1, new_sand_grain.1 + 1),
                Point(new_sand_grain.0 + 1, new_sand_grain.1 + 1),
            ];
            for neighbor in neighbors {
                if !sand_locations.contains(&neighbor) && !stone_locations.contains(&neighbor) {
                    new_sand_grain = neighbor;
                    continue 'falling_sand_grain;
                }
            }
            // The sand rests
            sand_locations.insert(new_sand_grain);
            break 'falling_sand_grain;
        }
    }

    final_answer(sand_locations.len(), submit, DAY, 1);
}

pub fn d14s2(submit: bool) {
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

    let mut sand_locations: HashSet<Point> = HashSet::new();

    'falling_sand_game: loop {
        let mut new_sand_grain = Point(DROP_POINT.0, DROP_POINT.1);
        'falling_sand_grain: loop {
            if new_sand_grain.1 == y_max {
                sand_locations.insert(new_sand_grain);
                break 'falling_sand_grain;
            }
            let neighbors = [
                Point(new_sand_grain.0, new_sand_grain.1 + 1),
                Point(new_sand_grain.0 - 1, new_sand_grain.1 + 1),
                Point(new_sand_grain.0 + 1, new_sand_grain.1 + 1),
            ];
            for neighbor in neighbors {
                if !sand_locations.contains(&neighbor) && !stone_locations.contains(&neighbor) {
                    new_sand_grain = neighbor;
                    continue 'falling_sand_grain;
                }
            }
            // The sand rests
            if new_sand_grain == DROP_POINT {
                sand_locations.insert(new_sand_grain);
                break 'falling_sand_game;
            } else {
                sand_locations.insert(new_sand_grain);
                break 'falling_sand_grain;
            }
        }
    }

    final_answer(sand_locations.len(), submit, DAY, 2);
}
