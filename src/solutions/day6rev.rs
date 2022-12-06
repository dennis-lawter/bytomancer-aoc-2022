use super::final_answer;
use super::input_raw;

const DAY: u8 = 6;

fn input() -> String {
    input_raw(DAY)
}

fn all_chars_unique(input: &[u8], start: usize, end: usize) -> bool {
    for i in start..end {
        for j in i + 1..end {
            if input[i] == input[j] {
                return false;
            }
        }
    }

    true
}

fn find_first_pattern_of_size(input: &[u8], size: usize) -> usize {
    for i in size..input.len() {
        if all_chars_unique(input, i - size, i) {
            return i;
        }
    }

    0
}

pub fn d6s1rev(submit: bool) {
    let input = input();
    let answer = find_first_pattern_of_size(input.as_bytes(), 4usize);
    final_answer(answer, submit, DAY, 1);
    assert_eq!(1566, answer);
}

pub fn d6s2rev(submit: bool) {
    let input = input();
    let answer = find_first_pattern_of_size(input.as_bytes(), 14usize);
    final_answer(answer, submit, DAY, 2);
    assert_eq!(2265, answer);
}
