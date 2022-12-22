use core::fmt;

use indexmap::IndexMap;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 22;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Path,
    Wall,
    Void,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Path => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Void => write!(f, " "),
        }
    }
}

#[derive(Debug)]
enum Facing {
    North,
    East,
    South,
    West,
}
impl Facing {
    fn score(&self) -> i32 {
        match self {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        }
    }
    fn cw(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }
}

#[derive(Debug)]
struct Player {
    position: (i32, i32),
    facing: Facing,
}

#[derive(Debug)]
struct Game {
    map: IndexMap<(i32, i32), Tile>,
    x_bound: usize,
    y_bound: usize,
    player: Player,
    directions: String,
}

fn input() -> Game {
    let raw = input_raw(DAY);
    let sections: Vec<String> = raw.split("\n\n").map(|item| item.to_owned()).collect();

    let map_raw = sections[0].clone();
    let map_raw_lines: Vec<String> = map_raw.split("\n").map(|item| item.to_owned()).collect();
    // println!("DEBUG: {:?}", map_raw_lines);
    // panic!("DIE");
    let mut longest_map_line = 0i32;
    let line_count = map_raw_lines.len();
    for line in map_raw_lines.iter() {
        if line.len() as i32 > longest_map_line {
            longest_map_line = line.len() as i32;
        }
    }

    let mut map: IndexMap<(i32, i32), Tile> = IndexMap::new();
    let mut starting_pos = (0i32, 0i32);
    let mut starting_pos_set = false;
    for y in 0..map_raw_lines.len() {
        let line = map_raw_lines.get(y).unwrap();
        let line_chars: Vec<char> = line.chars().collect();
        for x in 0..line_chars.len() {
            let line_char = line_chars[x];
            // print!("{}", line_char);
            match line_char {
                ' ' => {
                    map.insert((x as i32, y as i32), Tile::Void);
                }
                '#' => {
                    map.insert((x as i32, y as i32), Tile::Wall);
                }
                '.' => {
                    if starting_pos_set == false {
                        starting_pos = (x as i32, y as i32);
                        starting_pos_set = true;
                    }
                    map.insert((x as i32, y as i32), Tile::Path);
                }
                _ => {}
            }
        }

        // println!();
        for x in line_chars.len()..longest_map_line as usize {
            map.insert((x as i32, y as i32), Tile::Void);
        }
    }
    // panic!("DIE");
    let directions = sections[1].clone();

    let player = Player {
        position: starting_pos,
        facing: Facing::East,
    };

    Game {
        map,
        x_bound: longest_map_line as usize,
        y_bound: line_count,
        player,
        directions,
    }
}

fn follow_path(game: &mut Game) {
    const NUMERALS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut buffer: Vec<char> = Vec::new();
    let direction_chars: Vec<char> = game.directions.chars().collect();
    for direction_char in direction_chars.iter() {
        println!("{}", direction_char);
        match direction_char {
            'L' => {
                let buffer_string: String = buffer.into_iter().collect();
                let steps_to_take = str::parse::<i32>(buffer_string.as_str()).unwrap();
                take_steps(game, steps_to_take);
                game.player.facing = Facing::ccw(&game.player.facing);
                buffer = Vec::new();

                println!("Player: {:?}", game.player);
            }
            'R' => {
                let buffer_string: String = buffer.into_iter().collect();
                let steps_to_take = str::parse::<i32>(buffer_string.as_str()).unwrap();
                take_steps(game, steps_to_take);
                game.player.facing = Facing::cw(&game.player.facing);
                buffer = Vec::new();

                println!("Player: {:?}", game.player);
            }
            numeral => {
                assert!(NUMERALS.contains(&numeral));
                buffer.push(numeral.clone());
            }
        }
    }
    if direction_chars.len() > 0 {
        let buffer_string: String = buffer.into_iter().collect();
        let steps_to_take = str::parse::<i32>(buffer_string.as_str()).unwrap();
        take_steps(game, steps_to_take);
    }
}

fn take_steps(game: &mut Game, steps_to_take: i32) {
    for _ in 0..steps_to_take {
        let motion: (i32, i32);
        match game.player.facing {
            Facing::North => {
                motion = (0, -1);
            }
            Facing::East => {
                motion = (1, 0);
            }
            Facing::South => {
                motion = (0, 1);
            }
            Facing::West => {
                motion = (-1, 0);
            }
        }

        let mut check = (
            game.player.position.0 + motion.0,
            game.player.position.1 + motion.1,
        );
        if check.0 < 0 {
            check.0 = game.x_bound as i32 - 1;
        } else if check.0 >= game.x_bound as i32 {
            check.0 = 0;
        }
        if check.1 < 0 {
            check.1 = game.y_bound as i32 - 1;
        } else if check.1 >= game.y_bound as i32 {
            check.1 = 0;
        }
        match game.map.get(&check).unwrap() {
            Tile::Path => {
                game.player.position = check;
                println!("Step {:?}", game.player.position);
            }
            Tile::Wall => {
                println!("Wall at {:?}", check);
                return;
            }
            Tile::Void => {
                while *game.map.get(&check).unwrap() == Tile::Void {
                    check.0 += motion.0;
                    check.1 += motion.1;

                    if check.0 < 0 {
                        check.0 = game.x_bound as i32 - 1;
                    } else if check.0 >= game.x_bound as i32 {
                        check.0 = 0;
                    }
                    if check.1 < 0 {
                        check.1 = game.y_bound as i32 - 1;
                    } else if check.1 >= game.y_bound as i32 {
                        check.1 = 0;
                    }
                    println!("Whoosh {:?}", check);
                }
                match game.map.get(&check).unwrap() {
                    Tile::Path => {
                        game.player.position = check;
                    }
                    Tile::Wall => {
                        return;
                    }
                    Tile::Void => {}
                }
            }
        }
    }
}

fn print_map(game: &mut Game) {
    for y in 0..game.y_bound {
        for x in 0..game.x_bound {
            print!("{}", game.map.get(&(x as i32, y as i32)).unwrap());
        }
        println!();
    }
}

pub fn d22s1(submit: bool) {
    let mut game = input();

    // println!("Game:\n{:?}\n\n\n", game);

    print_map(&mut game);
    // panic!("DIE");

    println!("Player: {:?}", game.player);

    follow_path(&mut game);

    println!("Player: {:?}", game.player);

    let answer = 1000 * (game.player.position.1 + 1)
        + 4 * (game.player.position.0 + 1)
        + game.player.facing.score();

    final_answer(answer, submit, DAY, 1);
}

pub fn d22s2(submit: bool) {
    let input = input();
    final_answer("NaN", submit, DAY, 2);
}
