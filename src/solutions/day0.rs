use super::final_answer;
use super::input_raw;

const DAY: u8 = 00;

fn input() -> Vec<String> {
    let raw = input_raw(DAY);
    let lines = raw.split("\n").map(|item| item.to_owned()).collect();

    lines
}

pub fn d00s1(submit: bool) {
    let input = input();
    final_answer(input.len(), submit, DAY, 1);
}

pub fn d00s2(submit: bool) {
    let input = input();
    final_answer(input.len(), submit, DAY, 2);
}
