use std::fmt::Display;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 13;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Branch {
    List(Vec<Branch>),
    Numeral(u8),
}
impl Ord for Branch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match branch_comparison(self, other, false) {
            BranchCmp::Less => std::cmp::Ordering::Less,
            BranchCmp::Equal => std::cmp::Ordering::Equal,
            BranchCmp::Greater => std::cmp::Ordering::Greater,
        }
    }
}
impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match branch_comparison(self, other, false) {
            BranchCmp::Less => Some(std::cmp::Ordering::Less),
            BranchCmp::Equal => Some(std::cmp::Ordering::Equal),
            BranchCmp::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}
impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeral(number) => write!(f, "{}", number),
            Self::List(list) => {
                let string_vec: Vec<String> =
                    list.into_iter().map(|item| format!("{}", item)).collect();
                let result = string_vec.join(",");
                write!(f, "[{}]", result)
            }
        }
    }
}

struct StringPair(String, String);
struct BranchPair(Branch, Branch);

fn input_as_pairs() -> Vec<StringPair> {
    let raw = input_raw(DAY);
    let pairs: Vec<&str> = raw.split("\n\n").collect();
    let mut result = Vec::with_capacity(pairs.len());
    for pair in pairs {
        let mut pair_split = pair.split("\n");
        let left = pair_split.next().unwrap().to_owned();
        let right = pair_split.next().unwrap().to_owned();
        result.push(StringPair(left, right));
    }

    result
}

fn input_as_packets() -> Vec<String> {
    let pairs = input_as_pairs();
    let mut result = Vec::with_capacity(pairs.len() * 2);

    for pair in pairs {
        result.push(pair.0);
        result.push(pair.1);
    }

    result
}

fn string_to_branch(input: String) -> Branch {
    let input_chars: Vec<char> = input.chars().collect();
    let mut input_iter = 0usize;

    char_vector_to_branch(&input_chars, &mut input_iter)
}

fn string_pair_to_branch_pair(pair: &StringPair) -> BranchPair {
    let left = pair.0.to_string();
    let right = pair.1.to_string();
    BranchPair(string_to_branch(left), string_to_branch(right))
}

fn char_vector_to_branch(input_chars: &Vec<char>, i: &mut usize) -> Branch {
    let mut list: Vec<Branch> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();

    loop {
        *i += 1;
        match input_chars[*i] {
            ',' => {
                if buffer.len() > 0 {
                    let buffer_string: String = buffer.into_iter().collect();
                    list.push(Branch::Numeral(
                        str::parse::<u8>(buffer_string.as_str()).unwrap(),
                    ));
                    buffer = Vec::new();
                }
            }
            '[' => {
                list.push(char_vector_to_branch(input_chars, i));
            }
            ']' => {
                if buffer.len() > 0 {
                    let buffer_string: String = buffer.into_iter().collect();
                    list.push(Branch::Numeral(
                        str::parse::<u8>(buffer_string.as_str()).unwrap(),
                    ));
                }
                return Branch::List(list);
            }
            // '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            //     buffer.push(input_chars[*i]);
            // }
            // invalid => {
            //     panic!("What is this? {}", invalid);
            // }
            numeral => {
                buffer.push(numeral);
            }
        }
    }
}

// fn string_to_branch(input: String) -> Branch {
//     if input.starts_with("[") {
//         let inner: Vec<Branch> = Vec::new();
//         Branch::List(inner)
//     } else if input.starts_with("[") || input.ends_with("]") {
//         panic!("invalid state in string_to_branch: {}", input);
//     } else if input == "" {
//         Branch::None
//     } else {
//         Branch::Numeral(str::parse::<u8>(input.as_str()).unwrap())
//     }
// }

// fn branch_comparison(left: String, right: String) -> bool {
//     if left.starts_with("[") && right.starts_with("[") {
//         // do a list comparison...somehow
//         false //todo
//     } else if left.starts_with("[") && right.starts_with("]") {
//         true
//     } else if left.starts_with("[") {
//         // wrap the right in []s and go deeper
//         false //todo
//     } else if right.starts_with("[") && left.contains("]") {
//         false
//     } else if right.starts_with("[") {
//         // wrap the left in []s and go deeper
//         false //todo
//     } else {
//         // simply compare 2 numbers
//         // let left = str::parse::<u64>()
//         false
//     }
// }

enum BranchCmp {
    Less,
    Equal,
    Greater,
}

fn branch_comparison(left: &Branch, right: &Branch, debug_print: bool) -> BranchCmp {
    if debug_print {
        println!(
            "  Compare left:  {:?}\n  Compare right: {:?}\n",
            left, right
        );
    }
    match left {
        Branch::List(left_inner) => match right {
            Branch::List(right_inner) => {
                let mut left_iter = 0usize;
                let mut right_iter = 0usize;
                loop {
                    if left_iter >= left_inner.len() && right_iter >= right_inner.len() {
                        return BranchCmp::Equal;
                    } else if left_iter >= left_inner.len() {
                        return BranchCmp::Less;
                    } else if right_iter >= right_inner.len() {
                        return BranchCmp::Greater;
                    } else {
                        match branch_comparison(
                            &left_inner[left_iter],
                            &right_inner[right_iter],
                            debug_print,
                        ) {
                            BranchCmp::Less => {
                                return BranchCmp::Less;
                            }
                            BranchCmp::Greater => {
                                return BranchCmp::Greater;
                            }
                            _ => {
                                // continue traversing the lists
                            }
                        }
                        left_iter += 1;
                        right_iter += 1;
                    }
                }
            }
            Branch::Numeral(right_inner) => branch_comparison(
                left,
                &Branch::List(vec![Branch::Numeral(*right_inner)]),
                debug_print,
            ),
        },
        Branch::Numeral(left_inner) => match right {
            Branch::List(_right_inner) => branch_comparison(
                &Branch::List(vec![Branch::Numeral(*left_inner)]),
                right,
                debug_print,
            ),
            Branch::Numeral(right_inner) => {
                if left_inner == right_inner {
                    if debug_print {
                        println!("    =");
                    }
                    return BranchCmp::Equal;
                } else if left_inner < right_inner {
                    if debug_print {
                        println!("    <");
                    }
                    return BranchCmp::Less;
                } else {
                    if debug_print {
                        println!("    >");
                    }
                    return BranchCmp::Greater;
                }
            }
        },
    }
}

pub fn d13s1(submit: bool) {
    let input = input_as_pairs();
    let mut ordered_indices: Vec<usize> = Vec::new();
    for i in 0..input.len() {
        let index = i + 1;
        println!("== Pair {} ==", index);
        let branch_pair = string_pair_to_branch_pair(&input[i]);
        println!("left:  {:?}", branch_pair.0);
        println!("right: {:?}", branch_pair.1);
        println!("\n");
        match branch_comparison(&branch_pair.0, &branch_pair.1, true) {
            BranchCmp::Less => {
                println!("SMALLER");
                ordered_indices.push(index);
            }
            BranchCmp::Equal => {
                println!("EQUAL");
                ordered_indices.push(index);
            }
            _ => {
                println!("LARGER");
            }
        }
        println!("\n");
    }
    println!("Ordered pair indices: {:?}", ordered_indices);
    final_answer(ordered_indices.iter().sum::<usize>(), submit, DAY, 1);
}

pub fn d13s2(submit: bool) {
    let packets = input_as_packets();
    // packets.push("[[2]]".to_owned());
    // packets.push("[[6]]".to_owned());

    let first_divider = string_to_branch("[[2]]".to_owned());
    let second_divider = string_to_branch("[[6]]".to_owned());

    let mut branches: Vec<Branch> = Vec::with_capacity(packets.len());
    branches.push(first_divider.clone());
    branches.push(second_divider.clone());
    for packet in packets {
        branches.push(string_to_branch(packet));
    }
    branches.sort();
    let mut decoder_keys: Vec<usize> = Vec::with_capacity(2);
    println!("\n\n\n\n");
    for i in 0..branches.len() {
        let index = i + 1;
        println!("{}\n", branches[i]);
        match branch_comparison(&branches[i], &first_divider, false) {
            BranchCmp::Equal => {
                decoder_keys.push(index);
            }
            _ => {}
        }
        match branch_comparison(&branches[i], &second_divider, false) {
            BranchCmp::Equal => {
                decoder_keys.push(index);
            }
            _ => {}
        }
    }
    final_answer(decoder_keys[0] * decoder_keys[1], submit, DAY, 2);
}
