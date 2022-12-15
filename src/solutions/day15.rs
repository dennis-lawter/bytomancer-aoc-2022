// use std::cmp::max;
// use std::cmp::min;
use std::collections::VecDeque;
use std::ops::Range;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 15;

#[derive(Clone, Copy)]
struct Point(i64, i64);

#[derive(Clone, Copy)]
struct Sensor(Point, Point);
impl Sensor {
    fn new_from_str(input: &str) -> Self {
        let regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let captures = regex.captures(input).unwrap();
        let position = Point(
            str::parse::<i64>(captures.get(1).unwrap().as_str()).unwrap(),
            str::parse::<i64>(captures.get(2).unwrap().as_str()).unwrap(),
        );
        let closest_beacon = Point(
            str::parse::<i64>(captures.get(3).unwrap().as_str()).unwrap(),
            str::parse::<i64>(captures.get(4).unwrap().as_str()).unwrap(),
        );

        Self(position, closest_beacon)
    }
    fn distance_sensed(&self) -> i64 {
        let x = (self.1 .0 - self.0 .0).abs();
        let y = (self.1 .1 - self.0 .1).abs();
        x + y
    }
    fn ys_covered_in_x_line(&self, y: i64) -> Option<Range<i64>> {
        let distance_from_y_to_sensor = (y - self.0 .1).abs();
        let distance = self.distance_sensed();
        let area_affected = distance - distance_from_y_to_sensor;
        if area_affected < 0 {
            return None;
        }

        let x_left = self.0 .0 - area_affected;
        let x_right = self.0 .0 + area_affected;
        Some(x_left..x_right)
    }
}

fn input() -> Vec<Sensor> {
    let raw = input_raw(DAY);
    let lines: Vec<&str> = raw.split("\n").collect();
    let mut result: Vec<Sensor> = Vec::with_capacity(lines.len());
    for line in lines {
        result.push(Sensor::new_from_str(line));
    }

    result
}

fn sample(input: &Vec<Sensor>, y: i64, max: i64) -> Option<i64> {
    let mut stored_ranges: VecDeque<Range<i64>> = VecDeque::new();
    // const Y_CHECKED: i64 = 2_000_000;
    // const Y_CHECKED: i64 = 10;
    for sensor in input {
        let ys_covered = sensor.ys_covered_in_x_line(y);
        match ys_covered {
            None => {
                continue;
            }
            Some(new_range) => {
                insert_range_into_list(&mut stored_ranges, new_range);
            }
        }
    }
    for _ in 0..stored_ranges.len() {
        let range_result = stored_ranges.pop_front();
        match range_result {
            Some(range) => {
                insert_range_into_list(&mut stored_ranges, range);
            }
            None => {}
        }
    }
    if stored_ranges[0].start < 0 && stored_ranges[0].end > max {
        // println!("\n\n");
        // println!("NOTHING FOR Y={}", y);
        return None;
    }
    for mut x in 0..max {
        let mut found = false;
        for range in &stored_ranges {
            if range.contains(&x) || range.end == x {
                found = true;
                break;
            } else {
                x = range.end;
            }
        }
        if !found {
            return Some(x);
        }
    }
    println!("\n\n");
    println!("NOTHING FOR Y={}", y);
    return None;
}

fn insert_range_into_list(list: &mut VecDeque<Range<i64>>, new_range: Range<i64>) {
    for _ in 0..list.len() {
        let mut stored_range = list.pop_front().unwrap();
        let mut range_mutated: bool = false;
        if new_range.contains(&stored_range.start)
            || stored_range.contains(&new_range.start)
            || new_range.contains(&stored_range.end)
            || stored_range.contains(&new_range.end)
        {
            if new_range.start < stored_range.start {
                stored_range.start = new_range.start;
            }
            if new_range.end > stored_range.end {
                stored_range.end = new_range.end;
            }
            range_mutated = true;
        }
        if range_mutated {}
        list.push_back(stored_range);
        if range_mutated {
            return;
        }
    }
    // println!("NEW RANGE {} .. {}", &new_range.start, &new_range.end);
    list.push_back(new_range);
}

pub fn d15s1(submit: bool) {
    let input = input();
    let mut stored_ranges: VecDeque<Range<i64>> = VecDeque::new();
    const Y_CHECKED: i64 = 2_000_000;
    // const Y_CHECKED: i64 = 10;
    for sensor in &input {
        let ys_covered = sensor.ys_covered_in_x_line(Y_CHECKED);
        match ys_covered {
            None => {
                continue;
            }
            Some(new_range) => {
                insert_range_into_list(&mut stored_ranges, new_range);
            }
        }
    }
    for _ in 0..stored_ranges.len() {
        let range_result = stored_ranges.pop_front();
        match range_result {
            Some(range) => {
                insert_range_into_list(&mut stored_ranges, range);
            }
            None => {}
        }
    }
    let mut covered_positions = 0i64;
    for stored_range in stored_ranges {
        covered_positions += stored_range.end - stored_range.start;
    }
    final_answer(covered_positions, submit, DAY, 1);
}

pub fn d15s2(submit: bool) {
    let input = input();
    // let mut save_y = -1;
    let mut saved_pos: Option<Point> = None;
    const Y_MAX: i64 = 4_000_000;
    // const Y_MAX: i64 = 20;
    for y in 0..Y_MAX {
        match sample(&input, y, Y_MAX) {
            Some(x) => {
                saved_pos = Some(Point(x, y));
                break;
            }
            None => {}
        }
    }
    let saved_pos_found = saved_pos.unwrap();
    println!("{},{}", saved_pos_found.0, saved_pos_found.1);
    let answer = saved_pos_found.0 * 4000000 + saved_pos_found.1;
    final_answer(answer, submit, DAY, 2);
}
