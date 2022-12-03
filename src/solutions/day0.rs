use super::final_answer;
use super::input_raw;

const DAY: u8 = 0;

fn input() -> String {
    input_raw(DAY)
}

pub fn d00s1(submit: bool) {
    let input = input();
    final_answer(input.len(), submit, DAY, 1)
}

pub fn d00s2(submit: bool) {
    let input = input();
    final_answer(input.len(), submit, DAY, 2)
}
