use super::final_answer;
use super::input_raw;

const DAY: u8 = 20;

fn input() -> Vec<i64> {
    let raw = input_raw(DAY);
    let lines: Vec<String> = raw.split("\n").map(|item| item.to_owned()).collect();
    let mut result: Vec<i64> = Vec::with_capacity(lines.len());

    for line in lines.iter() {
        result.push(str::parse::<i64>(line.as_str()).unwrap())
    }

    result
}

#[derive(Debug)]
struct Datagram {
    number: i64,
    current_index: usize,
}

fn print_datagrams_in_order(datagrams: &Vec<Datagram>) {
    print!("Datagrams: ");
    for i in 0..datagrams.len() {
        for find_me in datagrams.iter() {
            if find_me.current_index == i {
                print!("{}@{} ", find_me.number, i);
            }
        }
    }
    println!();
}

fn mix(datagrams: &mut Vec<Datagram>) {
    let datagrams_len = datagrams.len();
    let last_datagram_index = datagrams_len - 1;
    for i in 0..datagrams_len {
        let target = datagrams.get_mut(i).unwrap();
        if target.number == 0 {
            continue;
        }
        let starting_index = target.current_index;
        let mut moving_to_index = (starting_index as i64) + target.number;
        moving_to_index %= last_datagram_index as i64;
        if moving_to_index <= 0 {
            moving_to_index += last_datagram_index as i64;
        }
        let moving_to_index = moving_to_index as usize;
        target.current_index = moving_to_index;
        for j in 0..datagrams_len {
            if i == j {
                continue;
            }
            let slider = datagrams.get_mut(j).unwrap();
            if starting_index < moving_to_index {
                if slider.current_index >= starting_index && slider.current_index <= moving_to_index
                {
                    slider.current_index -= 1;
                }
            } else {
                if slider.current_index <= starting_index && slider.current_index >= moving_to_index
                {
                    slider.current_index += 1;
                }
            }
        }
    }
}

pub fn d20s1(submit: bool) {
    let input = input();

    let mut datagrams: Vec<Datagram> = Vec::with_capacity(input.len());

    for i in 0..input.len() {
        let datagram = Datagram {
            number: input[i],
            current_index: i,
        };
        datagrams.push(datagram);
    }

    println!("Starting order:");
    print_datagrams_in_order(&datagrams);
    println!();
    mix(&mut datagrams);

    println!("FINAL RESULT:");
    print_datagrams_in_order(&datagrams);
    println!("\n\n");

    let mut zero_value_index = 0usize;
    for i in 0..datagrams.len() {
        if datagrams.get(i).unwrap().number == 0 {
            zero_value_index = datagrams.get(i).unwrap().current_index;
            break;
        }
    }
    let coord_one_index = (zero_value_index + 1_000) % datagrams.len();
    let coord_two_index = (zero_value_index + 2_000) % datagrams.len();
    let coord_three_index = (zero_value_index + 3_000) % datagrams.len();
    let mut coord_one_value = 0i64;
    let mut coord_two_value = 0i64;
    let mut coord_three_value = 0i64;
    for i in 0..datagrams.len() {
        if datagrams.get(i).unwrap().current_index == coord_one_index {
            coord_one_value = datagrams.get(i).unwrap().number;
        }
        if datagrams.get(i).unwrap().current_index == coord_two_index {
            coord_two_value = datagrams.get(i).unwrap().number;
        }
        if datagrams.get(i).unwrap().current_index == coord_three_index {
            coord_three_value = datagrams.get(i).unwrap().number;
        }
    }
    println!("zero_value_index: {:?}", zero_value_index);
    println!("coord_one_value: {:?}", coord_one_value);
    println!("coord_two_value: {:?}", coord_two_value);
    println!("coord_three_value: {:?}", coord_three_value);
    let sum = coord_one_value + coord_two_value + coord_three_value;

    final_answer(sum, submit, DAY, 1);
}

const DECRYPTION_KEY: i64 = 811589153;

pub fn d20s2(submit: bool) {
    let mut datagrams: Vec<Datagram> = Vec::with_capacity(input.len());

    for i in 0..input.len() {
        let datagram = Datagram {
            number: input[i] * DECRYPTION_KEY,
            current_index: i,
        };
        datagrams.push(datagram);
    }

    println!("Starting order:");
    print_datagrams_in_order(&datagrams);
    println!();
    for _ in 0..10 {
        mix(&mut datagrams);
    }

    println!("FINAL RESULT:");
    print_datagrams_in_order(&datagrams);
    println!("\n\n");

    let mut zero_value_index = 0usize;
    for i in 0..datagrams.len() {
        if datagrams.get(i).unwrap().number == 0 {
            zero_value_index = datagrams.get(i).unwrap().current_index;
            break;
        }
    }
    let coord_one_index = (zero_value_index + 1_000) % datagrams.len();
    let coord_two_index = (zero_value_index + 2_000) % datagrams.len();
    let coord_three_index = (zero_value_index + 3_000) % datagrams.len();
    let mut coord_one_value = 0i64;
    let mut coord_two_value = 0i64;
    let mut coord_three_value = 0i64;
    for i in 0..datagrams.len() {
        if datagrams.get(i).unwrap().current_index == coord_one_index {
            coord_one_value = datagrams.get(i).unwrap().number;
        }
        if datagrams.get(i).unwrap().current_index == coord_two_index {
            coord_two_value = datagrams.get(i).unwrap().number;
        }
        if datagrams.get(i).unwrap().current_index == coord_three_index {
            coord_three_value = datagrams.get(i).unwrap().number;
        }
    }
    println!("zero_value_index: {:?}", zero_value_index);
    println!("coord_one_value: {:?}", coord_one_value);
    println!("coord_two_value: {:?}", coord_two_value);
    println!("coord_three_value: {:?}", coord_three_value);
    let sum = coord_one_value + coord_two_value + coord_three_value;

    final_answer(sum, submit, DAY, 2);
}
