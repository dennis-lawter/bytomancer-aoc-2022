use super::final_answer;
use super::input_raw;

const DAY: u8 = 10;

fn input() -> Vec<String> {
    input_raw(DAY)
        .split("\n")
        .map(|item| item.to_owned())
        .collect()
}

pub fn d10s1(submit: bool) {
    let input = input();
    let mut x_register: i64 = 1;
    let mut cycle_count: i64 = 0;
    let mut signal_strength_sum: i64 = 0;

    for line in input {
        if line == "noop" {
            cycle_count += 1;
            if (cycle_count + 20) % 40 == 0 {
                signal_strength_sum += cycle_count * x_register;
            }
        } else {
            for _ in 0..2 {
                cycle_count += 1;
                if (cycle_count + 20) % 40 == 0 {
                    signal_strength_sum += cycle_count * x_register;
                }
            }
            let mut line_split = line.split(" ");
            let (_, value) = (
                line_split.next().unwrap(),
                str::parse::<i64>(line_split.next().unwrap()).unwrap(),
            );
            x_register += value;
        }
    }
    final_answer(signal_strength_sum, submit, DAY, 1);
}

fn print_pixel(cycle_count: i64, x_register: i64) {
    if x_register == cycle_count % 40
        || x_register + 1 == cycle_count % 40
        || x_register + 2 == cycle_count % 40
    {
        print!("█");
    } else {
        print!(" ")
    }
    if cycle_count % 40 == 0 {
        println!();
    }
}

pub fn d10s2(_submit: bool) {
    let input = input();
    let mut x_register: i64 = 1;
    let mut cycle_count: i64 = 0;

    for line in input {
        if line == "noop" {
            cycle_count += 1;
            print_pixel(cycle_count, x_register);
        } else {
            for _ in 0..2 {
                cycle_count += 1;
                print_pixel(cycle_count, x_register);
            }
            let mut line_split = line.split(" ");
            let (_, value) = (
                line_split.next().unwrap(),
                str::parse::<i64>(line_split.next().unwrap()).unwrap(),
            );
            x_register += value;
        }
    }
    println!()
}
