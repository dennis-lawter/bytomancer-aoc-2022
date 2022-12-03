use super::final_answer;
use super::input_raw;

const DAY: u8 = 3;

fn input() -> String {
    input_raw(DAY)
}

fn get_repeat_char(token: &str) -> char {
    for i in 0..token.len() / 2 {
        for j in token.len() / 2..token.len() {
            if token.chars().nth(i) == token.chars().nth(j) {
                return token.chars().nth(i).unwrap();
            }
        }
    }
    return 'a';
}

pub fn d3s1(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let mut score = 0u64;
    for token in tokens {
        let rep_char = get_repeat_char(token);
        let mut rep_priority = (rep_char as i64) - ('a' as i64) + 1;
        if rep_priority < 0 {
            rep_priority += 32 + 26;
        }
        score += rep_priority as u64;
    }

    final_answer(score, submit, DAY, 1)
}

fn char_in_three(a: &str, b: &str, c: &str) -> char {
    for i in 0..a.len() {
        for j in 0..b.len() {
            for k in 0..c.len() {
                if a.chars().nth(i) == b.chars().nth(j) && a.chars().nth(i) == c.chars().nth(k) {
                    return a.chars().nth(i).unwrap();
                }
            }
        }
    }
    return 'a';
}

pub fn d3s2(submit: bool) {
    let input = input();
    let tokens = input.split("\n").collect::<Vec<&str>>();
    let mut score = 0u64;
    let mut i = 0usize;
    while i < tokens.len() {
        let a = tokens[i];
        let b = tokens[i + 1];
        let c = tokens[i + 2];
        let rep_char = char_in_three(a, b, c);
        let mut rep_priority = (rep_char as i64) - ('a' as i64) + 1;
        if rep_priority < 0 {
            rep_priority += 32 + 26;
        }
        score += rep_priority as u64;
        i += 3;
    }

    final_answer(score, submit, DAY, 2)
}
