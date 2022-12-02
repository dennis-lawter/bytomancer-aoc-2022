use super::final_answer;
use super::input_raw;

const DAY: u8 = 2;

fn input() -> String {
    input_raw(DAY)
}

pub fn d2s1rev(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let mut score = 0u64;
    for token in tokens {
        let elf = (token.chars().nth(0).unwrap() as u64) - ('A' as u64) + 1;
        let you = (token.chars().nth(2).unwrap() as u64) - ('X' as u64) + 1;
        score += you as u64;
        let you_adj = you + 3;
        if you_adj - elf == 3 {
            score += 3u64;
        } else if (you_adj - elf) % 3 == 1 {
            score += 6u64;
        }
    }
    final_answer(score, submit, DAY, 1);

    assert!(11150 == score);
}

pub fn d2s2rev(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let mut score = 0u64;
    for token in tokens {
        let elf = (token.chars().nth(0).unwrap() as i64) - ('A' as i64) + 1;
        let result = token.chars().nth(2).unwrap();
        match result {
            'X' => {
                score += match elf {
                    1 => 3,
                    2 => 1,
                    3 => 2,
                    _ => 0,
                }
            }
            'Y' => {
                score += elf as u64 + 3u64;
            }
            'Z' => {
                score += match elf {
                    1 => 2 + 6,
                    2 => 3 + 6,
                    3 => 1 + 6,
                    _ => 0,
                }
            }
            _ => {}
        }
    }
    final_answer(score, submit, DAY, 2);

    assert!(8295 == score);
}
