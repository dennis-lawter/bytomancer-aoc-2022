use std::collections::HashMap;
use std::collections::VecDeque;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 21;

#[derive(Clone)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
}
impl Operation {
    fn from_str(input: &str) -> Self {
        match input {
            "+" => Self::ADD,
            "-" => Self::SUB,
            "*" => Self::MUL,
            "/" => Self::DIV,
            invalid => panic!("Invalid operator: {}", invalid),
        }
    }
}

struct Monkey {
    name: String,
    value: Option<i64>,
    left_param: Option<String>,
    right_param: Option<String>,
    operation: Option<Operation>,
}
impl Monkey {
    fn get_left_param(&self) -> String {
        match &self.left_param {
            Some(param) => param.clone(),
            None => "".to_owned(),
        }
    }
    fn get_right_param(&self) -> String {
        match &self.right_param {
            Some(param) => param.clone(),
            None => "".to_owned(),
        }
    }
    fn get_operation(&self) -> Option<Operation> {
        match &self.operation {
            Some(operation) => Some(operation.clone()),
            None => None,
        }
    }
}

fn input() -> HashMap<String, Monkey> {
    let raw = input_raw(DAY);
    let lines: Vec<String> = raw.split("\n").map(|item| item.to_owned()).collect();
    let regex_yeller = Regex::new(r"(\w{4}): (\d+)").unwrap();
    let regex_operator = Regex::new(r"(\w{4}): (\w{4}) ([\+\-\*/]) (\w{4})").unwrap();

    let mut result: HashMap<String, Monkey> = HashMap::with_capacity(lines.len());

    for line in &lines {
        // println!("LINE: {}", line);
        if regex_yeller.is_match(line.as_str()) {
            let captures = regex_yeller.captures(line.as_str()).unwrap();
            let monkey = Monkey {
                name: captures.get(1).unwrap().as_str().to_owned(),
                value: Some(str::parse::<i64>(captures.get(2).unwrap().as_str()).unwrap()),
                left_param: None,
                right_param: None,
                operation: None,
            };
            result.insert(monkey.name.clone(), monkey);
        } else if regex_operator.is_match(line.as_str()) {
            let captures = regex_operator.captures(line.as_str()).unwrap();
            let monkey = Monkey {
                name: captures.get(1).unwrap().as_str().to_owned(),
                value: None,
                left_param: Some(captures.get(2).unwrap().as_str().to_owned()),
                right_param: Some(captures.get(4).unwrap().as_str().to_owned()),
                operation: Some(Operation::from_str(captures.get(3).unwrap().as_str())),
            };
            result.insert(monkey.name.clone(), monkey);
        } else {
            panic!("No regex matches");
        }
    }

    result
}

pub fn d21s1(submit: bool) {
    let monkeys = input();

    let values = process_yells(&monkeys);

    // println!("{:?}", values);

    final_answer(values["root"], submit, DAY, 1);
}

fn process_yells(monkeys: &HashMap<String, Monkey>) -> HashMap<String, i64> {
    let mut names_unprocessed: VecDeque<String> = monkeys.keys().map(|item| item.clone()).collect();
    let mut values: HashMap<String, i64> = HashMap::new();
    let mut no_change_counter = 0usize;
    while no_change_counter <= names_unprocessed.len() && names_unprocessed.is_empty() != true {
        let name = names_unprocessed.pop_front().unwrap();
        // println!("Checking {}", name);
        let subject = monkeys.get(&name).unwrap();
        match subject.value {
            Some(value) => {
                no_change_counter = 0;
                values.insert(name, value);
                continue; // do not push
            }
            None => {
                let left_param = subject.get_left_param();
                let right_param = subject.get_right_param();
                if values.contains_key(&left_param) && values.contains_key(&right_param) {
                    let arg1 = values[&left_param];
                    let arg2 = values[&right_param];
                    let operation = subject.get_operation().unwrap();
                    let new_value: i64;
                    match operation {
                        Operation::ADD => new_value = arg1 + arg2,
                        Operation::SUB => new_value = arg1 - arg2,
                        Operation::MUL => new_value = arg1 * arg2,
                        Operation::DIV => new_value = arg1 / arg2,
                        Operation::EQ => {
                            no_change_counter += 1;
                            names_unprocessed.push_back(name);
                            continue;
                        }
                    }
                    no_change_counter = 0;
                    values.insert(name, new_value);
                    continue; // do not push
                }
            }
        }
        no_change_counter += 1;
        names_unprocessed.push_back(name);
    }

    values
}

pub fn d21s2(submit: bool) {
    let mut monkeys = input();
    {
        let root = monkeys.get_mut("root").unwrap();
        root.operation = Some(Operation::EQ);
        root.value = None;
    }
    {
        let humn = monkeys.get_mut("humn").unwrap();
        humn.value = None;
    }

    let values = process_yells(&monkeys);

    let mut known = 0i64;
    {
        let root = monkeys.get("root").unwrap();
        let left = root.get_left_param();
        let right = root.get_right_param();
        if values.contains_key(&left) {
            known = values[&left];
        } else if values.contains_key(&right) {
            known = values[&right];
        }
    }

    let answer = solve_for_humn("root".to_owned(), known, &monkeys, &values);

    final_answer(answer, submit, DAY, 2);
}

fn solve_for_humn(
    name: String,
    known: i64,
    monkeys: &HashMap<String, Monkey>,
    values: &HashMap<String, i64>,
) -> i64 {
    println!("{} knows {}", name, known);
    if name == "humn".to_owned() {
        return known;
    }

    let knowing_monkey = &monkeys[&name];
    let learning_monkey_name;
    let new_known;
    let left = knowing_monkey.get_left_param();
    let right = knowing_monkey.get_right_param();
    let is_known_operand_left: bool; // For not communicative math
    let known_operand;
    if values.contains_key(&right) {
        learning_monkey_name = left;
        known_operand = values[&right];
        is_known_operand_left = false;
    } else if values.contains_key(&left) {
        learning_monkey_name = right;
        known_operand = values[&left];
        is_known_operand_left = true;
    } else {
        panic!("{} doesn't know enough!", name);
    }

    // known = left _ right
    match knowing_monkey.get_operation().unwrap() {
        Operation::ADD => {
            // known = left + right
            // left = known - right
            // right = known - left
            new_known = known - known_operand;
        }
        Operation::SUB => {
            // known = left - right
            // left = known + right
            // right = left - known
            if !is_known_operand_left {
                new_known = known + known_operand;
            } else {
                new_known = known_operand - known;
            }
        }
        Operation::MUL => {
            // known = left * right
            // left = known / right
            // right = known / left
            new_known = known / known_operand;
        }
        Operation::DIV => {
            // known = left / right
            // left = known * right
            // right = left / known
            if !is_known_operand_left {
                new_known = known * known_operand
            } else {
                new_known = known_operand / known;
            }
        }
        Operation::EQ => new_known = known,
    }

    solve_for_humn(learning_monkey_name, new_known, monkeys, values)
}
