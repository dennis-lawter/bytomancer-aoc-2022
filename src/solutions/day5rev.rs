use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 5;

struct Puzzle {
    pub stacks: Vec<Vec<char>>,
    pub commands: Vec<String>,
}

fn input() -> Puzzle {
    let input = input_raw(DAY);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut first_split = input.split("\n\n");
    let box_string = first_split.next().unwrap();
    let mut box_tokens: Vec<&str> = box_string.split("\n").collect();
    let labels = box_tokens.pop().unwrap();
    let label_regex = Regex::new(r" *(\d+) *").unwrap();
    let label_captures = label_regex.captures_iter(labels);
    for _ in label_captures {
        stacks.push(Vec::new());
    }
    box_tokens.reverse();
    for box_token in box_tokens {
        let mut i = 0usize;
        let chars: Vec<char> = box_token.chars().collect();
        while i < stacks.len() {
            let char = chars[i * 4 + 1];
            if char != ' ' {
                stacks[i].push(char);
            }
            i += 1;
        }
    }
    println!("{:?}", stacks);
    let command_string = first_split.next().unwrap();
    let commands: Vec<String> = command_string.split("\n").map(|a| a.to_string()).collect();

    Puzzle { stacks, commands }
}

pub fn d5s1rev(submit: bool) {
    let mut puz = input();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for command in puz.commands {
        let captures = regex.captures(command.as_str()).unwrap();
        let mov = str::parse::<usize>(captures.get(1).unwrap().as_str()).unwrap();
        let src = str::parse::<usize>(captures.get(2).unwrap().as_str()).unwrap() - 1usize;
        let dst = str::parse::<usize>(captures.get(3).unwrap().as_str()).unwrap() - 1usize;
        for _ in 0..mov {
            let tmp = puz.stacks[src].pop().unwrap();
            puz.stacks[dst].push(tmp);
        }
    }

    let mut top_of_stacks = String::from("");
    for mut stack in puz.stacks {
        let top_char = stack.pop().unwrap();
        top_of_stacks.push(top_char);
    }

    final_answer(top_of_stacks.as_str(), submit, DAY, 1);

    assert_eq!("SHQWSRBDL", top_of_stacks.as_str());
}

pub fn d5s2rev(submit: bool) {
    let mut puz = input();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for command in puz.commands {
        let captures = regex.captures(command.as_str()).unwrap();
        let mov = str::parse::<usize>(captures.get(1).unwrap().as_str()).unwrap();
        let src = str::parse::<usize>(captures.get(2).unwrap().as_str()).unwrap() - 1usize;
        let dst = str::parse::<usize>(captures.get(3).unwrap().as_str()).unwrap() - 1usize;
        let mut tmp_stack: Vec<char> = Vec::new();
        for _ in 0..mov {
            let tmp = puz.stacks[src].pop().unwrap();
            tmp_stack.push(tmp);
        }
        while tmp_stack.len() > 0 {
            let tmp = tmp_stack.pop().unwrap();
            puz.stacks[dst].push(tmp);
        }
    }

    let mut top_of_stacks = String::from("");
    for mut stack in puz.stacks {
        let top_char = stack.pop().unwrap();
        top_of_stacks.push(top_char);
    }
    final_answer(top_of_stacks.as_str(), submit, DAY, 2);

    assert_eq!("CDTQZHBRS", top_of_stacks.as_str());
}
