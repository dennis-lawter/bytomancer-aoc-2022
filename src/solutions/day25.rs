use super::final_answer;
use super::input_raw;

const DAY: u8 = 25;

fn input() -> Vec<String> {
    let raw = input_raw(DAY);
    let lines = raw.split("\n").map(|item| item.to_owned()).collect();

    lines
}

pub fn d25s1(submit: bool) {
    let lines = input();
    let mut sum = 0i128;
    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        let base: i128 = 5;
        let mut place_value = base.pow(chars.len() as u32 - 1);
        let mut accum = 0i128;
        for i in 0..chars.len() {
            match chars[i] {
                '2' => {
                    accum += 2 * place_value;
                }
                '1' => {
                    accum += place_value;
                }
                '-' => {
                    accum -= place_value;
                }
                '=' => {
                    accum -= place_value * 2;
                }
                '0' => {}
                _ => {}
            }
            place_value = place_value / 5;
        }

        println!("{} = {}", line, accum);
        sum += accum;
    }
    println!("Sum in decimal: {}", sum);
    let mut snafu_chars: Vec<char> = Vec::new();
    let mut carry = 0i128;
    while (sum + carry) > 0 {
        match (sum + carry) % 5 {
            0 => {
                snafu_chars.push('0');
                carry = 0;
            }
            1 => {
                snafu_chars.push('1');
                carry = 0;
            }
            2 => {
                snafu_chars.push('2');
                carry = 0;
            }
            3 => {
                snafu_chars.push('=');
                carry = 1;
            }
            4 => {
                snafu_chars.push('-');
                carry = 1;
            }
            _ => {}
        }
        sum = sum / 5;
    }
    snafu_chars.reverse();
    let snafu_answer: String = snafu_chars.into_iter().collect();
    final_answer(snafu_answer, submit, DAY, 1);
}
