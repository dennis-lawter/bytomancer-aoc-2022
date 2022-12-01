use super::final_answer;
use super::input_raw;

const DAY: u8 = 1;

fn input() -> String {
    input_raw(DAY)
}

pub fn d1s1(submit: bool) {
    let input = format!("{}{}", input(), "\n");
    let tokens = input.split("\n");
    let mut greatest = 0u32;
    let mut sum = 0u32;
    for token in tokens {
        if token.trim() == "" {
            println!();
            println!("Sum: {}", sum);
            if sum > greatest {
                greatest = sum;
            }
            sum = 0u32;
        } else {
            let num = token.parse::<u32>().unwrap();
            sum += num;
            print!("{} ", sum);
        }
    }
    final_answer(greatest, submit, DAY, 1);
}

pub fn d1s2(submit: bool) {
    let input = format!("{}{}", input(), "\n");
    let tokens = input.split("\n");
    let mut sum = 0u32;
    let mut sums: Vec<u32> = Vec::<u32>::new();
    for token in tokens {
        if token.trim() == "" {
            println!();
            println!("Sum: {}", sum);
            sums.push(sum);
            sum = 0u32;
        } else {
            let num = token.parse::<u32>().unwrap();
            sum += num;
            print!("{} ", token);
        }
    }
    sums.sort_unstable();
    sums.reverse();

    println!("{:?}", sums);

    final_answer(sums[0] + sums[1] + sums[2], submit, DAY, 2);
}
