use super::final_answer;
use super::input_raw;

const DAY: u8 = 2;

fn input() -> String {
    input_raw(DAY)
}

pub fn d2s1(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let mut score = 0u64;
    for token in tokens {
        let elf = token.chars().nth(0).unwrap();
        let you = token.chars().nth(2).unwrap();
        match you {
            'X' => {
                score += 1;
                match elf {
                    'A' => {
                        score += 3;
                    }
                    'B' => {
                        score += 0;
                    }
                    'C' => {
                        score += 6;
                    }
                    _ => {}
                }
            }
            'Y' => {
                score += 2;
                match elf {
                    'A' => {
                        score += 6;
                    }
                    'B' => {
                        score += 3;
                    }
                    'C' => {
                        score += 0;
                    }
                    _ => {}
                }
            }
            'Z' => {
                score += 3;
                match elf {
                    'A' => {
                        score += 0;
                    }
                    'B' => {
                        score += 6;
                    }
                    'C' => {
                        score += 3;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    final_answer(score, submit, DAY, 1)
}

pub fn d2s2(submit: bool) {
    let input = input();
    let tokens = input.split("\n");
    let mut score = 0u64;
    for token in tokens {
        let elf = token.chars().nth(0).unwrap();
        let you = token.chars().nth(2).unwrap();
        match you {
            'X' => match elf {
                'A' => {
                    score += 3;
                }
                'B' => {
                    score += 1;
                }
                'C' => {
                    score += 2;
                }
                _ => {}
            },
            'Y' => match elf {
                'A' => {
                    score += 4;
                }
                'B' => {
                    score += 5;
                }
                'C' => {
                    score += 6;
                }
                _ => {}
            },
            'Z' => match elf {
                'A' => {
                    score += 8;
                }
                'B' => {
                    score += 9;
                }
                'C' => {
                    score += 7;
                }
                _ => {}
            },
            _ => {}
        }
    }
    final_answer(score, submit, DAY, 2)
}
