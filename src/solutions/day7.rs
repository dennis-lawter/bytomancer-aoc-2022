use core::fmt;
use std::str::FromStr;

use indexmap::IndexMap;
use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 7;

fn input() -> Box<ElfDir> {
    let input: Vec<String> = input_raw(DAY).split("\n").map(|a| a.to_string()).collect();
    let cmd_regex = Regex::new(r"\$ (\w+) ?(.*)").unwrap();
    let dir_regex = Regex::new(r"dir (.+)").unwrap();
    let file_regex = Regex::new(r"(\d+) (.+)").unwrap();

    let mut pwd: Vec<String> = Vec::new();
    let mut root_dir: Box<ElfDir> = Box::new(ElfDir::new("/"));
    let mut i = 0usize;

    while i < input.len() {
        let line = input[i].clone();
        if cmd_regex.is_match(&line) {
            let captures = cmd_regex.captures(&line).unwrap();
            let cmd = captures.get(1).unwrap().as_str();
            let arg = captures.get(2).unwrap().as_str();
            match Command::from_str(cmd).unwrap() {
                Command::Ls => {
                    let mut tmp_path = pwd.clone();
                    tmp_path.remove(0);
                    // println!("Seeking: {:?}", tmp_path);
                    let cur_dir = get_dir(&mut root_dir, &mut tmp_path);
                    let mut peek = input[i + 1].clone();
                    while !cmd_regex.is_match(peek.as_str()) {
                        if dir_regex.is_match(peek.as_str()) {
                            let captures = dir_regex.captures(peek.as_str()).unwrap();
                            let name = captures.get(1).unwrap().as_str();
                            cur_dir.add_dir(name);
                        } else if file_regex.is_match(peek.as_str()) {
                            let captures = file_regex.captures(peek.as_str()).unwrap();
                            let size =
                                str::parse::<usize>(captures.get(1).unwrap().as_str()).unwrap();
                            let name = captures.get(2).unwrap().as_str();
                            cur_dir.add_file(name, size);
                        } else {
                            panic!("Unrecognized resource: {}", peek.as_str());
                        }
                        i += 1;
                        if i >= input.len() - 1 {
                            break;
                        }
                        peek = input[i + 1].clone();
                    }
                }
                Command::Cd => match arg {
                    "/" => {
                        pwd = Vec::new();
                        pwd.push("/".to_string());
                    }
                    ".." => {
                        pwd.pop();
                    }
                    _ => {
                        pwd.push(arg.to_string());
                    }
                },
            }
        } else {
            panic!("Unrecognized parse: {}", line);
        }
        i += 1;
    }

    println!("{}", root_dir);

    root_dir
}

enum Command {
    Ls,
    Cd,
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ls => write!(f, "ls"),
            Self::Cd => write!(f, "cd"),
        }
    }
}
impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ls" => Ok(Self::Ls),
            "cd" => Ok(Self::Cd),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ElfFile {
    pub name: String,
    pub size: usize,
}
impl fmt::Display for ElfFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <8} {}", self.size, self.name)
    }
}

#[derive(Debug)]
struct ElfDir {
    pub name: String,
    pub dirs: IndexMap<String, Box<ElfDir>>,
    pub files: IndexMap<String, ElfFile>,
}
impl fmt::Display for ElfDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", self.to_string_with_depth(0)))
    }
}
impl ElfDir {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dirs: IndexMap::new(),
            files: IndexMap::new(),
        }
    }
    fn to_string_with_depth(&self, depth: usize) -> String {
        let stripped_name = self.name.trim_end_matches("/");
        let mut result = format!("{: <1$}{2}/", "", depth * 2, stripped_name);
        for (_, child) in &self.dirs {
            result = format!("{}\n{}", result, child.to_string_with_depth(depth + 1));
        }
        for (_, child) in &self.files {
            result = format!("{2}\n{: <1$}{3}", "", (depth + 1) * 2, result, child);
        }

        result
    }
    pub fn add_dir(&mut self, name: &str) {
        self.dirs
            .insert(name.to_string(), Box::new(Self::new(name)));
    }
    pub fn add_file(&mut self, name: &str, size: usize) {
        self.files.insert(
            name.to_string(),
            ElfFile {
                name: name.to_string(),
                size,
            },
        );
    }
    pub fn get_recursive_size(&self) -> usize {
        let mut counter = 0usize;
        for (_, dir) in &self.dirs {
            counter += dir.get_recursive_size();
        }
        for (_, file) in &self.files {
            counter += file.size;
        }

        counter
    }
    pub fn get_all_dirs_flat(&self) -> Vec<&ElfDir> {
        if self.dirs.len() == 0 {
            Vec::new()
        } else {
            let mut dir_vec = Vec::new();
            for (_, dir) in self.dirs.iter() {
                for child in dir.as_ref().get_all_dirs_flat() {
                    dir_vec.push(child);
                }
                dir_vec.push(dir.as_ref());
            }
            dir_vec
        }
    }
}

fn get_dir<'a>(cur_dir: &'a mut Box<ElfDir>, path: &'a mut Vec<String>) -> &'a mut Box<ElfDir> {
    if path.len() <= 0 {
        // assert path == name
        // cur_dir.dirs.get_mut(path[0].as_str()).unwrap()
        cur_dir
    } else {
        let dir_name = path[0].clone();
        path.remove(0);
        // println!("Currently in {}: {:?}\n", &dir_name, cur_dir.dirs);
        let target = cur_dir.dirs.get_mut(&dir_name).unwrap();
        get_dir(target, path)
    }
}

pub fn d7s1(submit: bool) {
    let input = input();

    let flat_dir_refs = input.get_all_dirs_flat();
    let mut small_dir_total = 0usize;
    for dir_ref in flat_dir_refs {
        let recursive_size = dir_ref.get_recursive_size();
        if recursive_size <= 100000 {
            small_dir_total += recursive_size
        }
    }

    final_answer(small_dir_total, submit, DAY, 1);
}

const FS_SIZE: usize = 70000000;
const FS_REQ: usize = 30000000;

pub fn d7s2(submit: bool) {
    let input = input();

    let flat_dir_refs = input.get_all_dirs_flat();
    let space_avail = FS_SIZE - input.get_recursive_size();
    let mut dir_sizes: Vec<usize> = Vec::with_capacity(flat_dir_refs.len());
    for dir_ref in flat_dir_refs {
        let recursive_size = dir_ref.get_recursive_size();
        dir_sizes.push(recursive_size);
    }
    dir_sizes.sort();
    for try_size in dir_sizes {
        if space_avail + try_size >= FS_REQ {
            final_answer(try_size, submit, DAY, 2);
            return;
        }
    }

    final_answer(space_avail, submit, DAY, 2);
}
