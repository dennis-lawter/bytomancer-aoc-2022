use crate::{input::get_input_as_string, solutions::final_answer};

fn input_raw() -> String {
    get_input_as_string("https://adventofcode.com/2022/day/1/input")
}

fn input() -> String {
    input_raw()
}

pub fn d1s1() {
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
    final_answer(greatest);
}

pub fn d1s2() {
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

    final_answer(sums[0] + sums[1] + sums[2]);
}
