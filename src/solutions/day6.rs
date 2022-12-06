use super::final_answer;
use super::input_raw;
use substring::Substring;

const DAY: u8 = 6;

fn input() -> String {
    input_raw(DAY)
}

fn all_chars_unique(input: &[u8]) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i] == input[j] {
                return false;
            }
        }
    }

    true
}

fn find_first_pattern_of_size(input: &str, size: usize) -> usize {
    for i in size..input.len() {
        let test = input.substring(i - size, i).as_bytes();
        if all_chars_unique(test) {
            println!("{}", input.substring(i - size, i));
            return i;
        }
    }

    0
}

pub fn d6s1(submit: bool) {
    let input = input();
    let answer = find_first_pattern_of_size(input.as_str(), 4usize);
    final_answer(answer, submit, DAY, 1)
}

pub fn d6s2(submit: bool) {
    let input = input();
    let answer = find_first_pattern_of_size(input.as_str(), 14usize);
    final_answer(answer, submit, DAY, 2)
}
