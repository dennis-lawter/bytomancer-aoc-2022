use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 4;

fn input() -> String {
    input_raw(DAY)
}

pub fn d4s1(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0usize;
    for token in tokens {
        let captures = regex.captures(token).unwrap();
        let l1 = str::parse::<u32>(captures.get(1).unwrap().as_str()).unwrap();
        let l2 = str::parse::<u32>(captures.get(3).unwrap().as_str()).unwrap();
        let r1 = str::parse::<u32>(captures.get(2).unwrap().as_str()).unwrap();
        let r2 = str::parse::<u32>(captures.get(4).unwrap().as_str()).unwrap();
        if l1 <= l2 && r1 >= r2 {
            // 1 contains 2
            count += 1;
            println!("{}", token);
        } else if l2 <= l1 && r2 >= r1 {
            // 2 contains 1
            count += 1;
            println!("{}", token);
        }
    }
    final_answer(count, submit, DAY, 1)
}

pub fn d4s2(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0usize;
    for token in tokens {
        let captures = regex.captures(token).unwrap();
        let l1 = str::parse::<u32>(captures.get(1).unwrap().as_str()).unwrap();
        let l2 = str::parse::<u32>(captures.get(3).unwrap().as_str()).unwrap();
        let r1 = str::parse::<u32>(captures.get(2).unwrap().as_str()).unwrap();
        let r2 = str::parse::<u32>(captures.get(4).unwrap().as_str()).unwrap();
        if l1 >= l2 && l1 <= r2 {
            count += 1;
            println!("{}", token);
        } else if r1 >= l2 && r1 <= r2 {
            count += 1;
            println!("{}", token);
        } else if l2 >= l1 && l2 <= r1 {
            count += 1;
            println!("{}", token);
        } else if r2 >= l1 && r2 <= r1 {
            count += 1;
            println!("{}", token);
        }
    }
    final_answer(count, submit, DAY, 2)
}
