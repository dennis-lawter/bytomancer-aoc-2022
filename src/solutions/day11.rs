use std::collections::VecDeque;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 11;

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: char,
    op_arg: Option<u64>,
    test_divisor: u64,
    test_true_target: usize,
    test_false_target: usize,
}
impl Monkey {
    pub fn new(input: &str) -> Self {
        let mut input_lines = input.split("\n");
        let id_regex = Regex::new(r"Monkey (\d):").unwrap();
        let starting_items_regex = Regex::new(r" (\d+)").unwrap();
        let operation_regex = Regex::new(r"Operation: new = old (.) (.+)").unwrap();
        let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        let test_true_regex = Regex::new(r"If true: throw to monkey (\d)").unwrap();
        let test_false_regex = Regex::new(r"If false: throw to monkey (\d)").unwrap();

        let id_capture = id_regex
            .captures(input_lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap();
        let starting_items_captures =
            starting_items_regex.captures_iter(input_lines.next().unwrap());
        let operation_capture_result = operation_regex
            .captures(input_lines.next().unwrap())
            .unwrap();
        let operation_capture = operation_capture_result.get(1).unwrap();
        let op_arg_capture = operation_capture_result.get(2).unwrap();
        let test_capture = test_regex
            .captures(input_lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap();
        let test_true_capture = test_true_regex
            .captures(input_lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap();
        let test_false_capture = test_false_regex
            .captures(input_lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap();

        let id = str::parse::<usize>(id_capture.as_str()).unwrap();
        let operation = operation_capture.as_str().chars().collect::<Vec<char>>()[0];
        let op_arg_raw = str::parse::<u64>(op_arg_capture.as_str());
        let op_arg: Option<u64>;
        match op_arg_raw {
            Ok(parsed) => {
                op_arg = Some(parsed);
            }
            Err(_) => {
                op_arg = None;
            }
        }
        let test_divisor = str::parse::<u64>(test_capture.as_str()).unwrap();
        let test_true_target = str::parse::<usize>(test_true_capture.as_str()).unwrap();
        let test_false_target = str::parse::<usize>(test_false_capture.as_str()).unwrap();

        let mut monkey = Self {
            id,
            items: VecDeque::new(),
            operation,
            op_arg,
            test_divisor,
            test_true_target,
            test_false_target,
        };

        for item_cap in starting_items_captures {
            monkey
                .items
                .push_back(str::parse::<u64>(item_cap.get(1).unwrap().as_str()).unwrap());
        }

        monkey
    }
}

fn play_game(
    mut monkeys: VecDeque<Monkey>,
    round_count: usize,
    worry_division: u64,
    print_enabled: bool,
) -> Vec<usize> {
    let monkey_count = monkeys.len();
    let mut inspection_count = vec![0usize; monkey_count];

    let mut common_divisor = worry_division;
    for monkey in &monkeys {
        common_divisor *= monkey.test_divisor;
    }

    for _round in 0..round_count {
        for _monkey_id in 0..monkey_count {
            let mut monkey = monkeys.pop_front().unwrap();

            if print_enabled {
                println!("Monkey {}:", monkey.id);
            }
            let items = &mut monkey.items;
            inspection_count[monkey.id] += items.len();
            while items.len() > 0 {
                let mut item = items.pop_front().unwrap();
                if print_enabled {
                    println!("  Monkey inspects an item with a worry level of {}.", item);
                }
                match monkey.operation {
                    '+' => match monkey.op_arg {
                        Some(op_arg) => {
                            item += op_arg;
                        }
                        None => {
                            item += item;
                        }
                    },
                    '*' => match monkey.op_arg {
                        Some(op_arg) => {
                            item *= op_arg;
                        }
                        None => {
                            item *= item;
                        }
                    },
                    invalid_op => {
                        panic!("Invalid operation: {}", invalid_op);
                    }
                }
                if print_enabled {
                    println!("    Monkey changes worry level to {}.", item);
                }
                item /= worry_division;
                item %= common_divisor;
                if print_enabled {
                    println!("    Monkey gets bored, level divided by 3 to {}.", item);
                }

                if item % monkey.test_divisor == 0 {
                    if print_enabled {
                        println!(
                            "    Current worry level is divisible by {}.",
                            monkey.test_divisor
                        );
                    }
                    let mut i = 0;

                    while monkeys[i].id != monkey.test_true_target {
                        i += 1;
                    }

                    monkeys.get_mut(i).unwrap().items.push_back(item);
                } else {
                    if print_enabled {
                        println!(
                            "    Current worry level is divisible by {}.",
                            monkey.test_divisor
                        );
                    }
                    let mut i = 0;

                    while monkeys[i].id != monkey.test_false_target {
                        i += 1;
                    }

                    monkeys.get_mut(i).unwrap().items.push_back(item);
                }
            }

            monkeys.push_back(monkey);
        }
    }

    inspection_count
}

fn input() -> VecDeque<Monkey> {
    let raw = input_raw(DAY);
    let monkey_strings: Vec<&str> = raw.split("\n\n").collect();
    let mut result: VecDeque<Monkey> = VecDeque::with_capacity(monkey_strings.len());
    for monkey_string in monkey_strings {
        result.push_back(Monkey::new(monkey_string));
    }

    result
}

pub fn d11s1(submit: bool) {
    let monkeys = input();
    let mut inspection_count = play_game(monkeys, 20, 3, true);

    inspection_count.sort();
    inspection_count.reverse();
    let monkey_business = inspection_count[0] * inspection_count[1];
    final_answer(monkey_business, submit, DAY, 1);
}

pub fn d11s2(submit: bool) {
    let monkeys = input();
    let mut inspection_count = play_game(monkeys, 10_000, 1, false);

    let mut i = 0;
    for inspection in &inspection_count {
        println!("Monkey {} inspected items {} times.", i, inspection);
        i += 1;
    }

    inspection_count.sort();
    inspection_count.reverse();
    let monkey_business = inspection_count[0] * inspection_count[1];
    final_answer(monkey_business, submit, DAY, 2);
}
