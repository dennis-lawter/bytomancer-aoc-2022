use std::cmp::max;
use std::collections::VecDeque;

use indexmap::IndexMap;
use pad::PadStr;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 22;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Path,
    Wall,
}

#[derive(Debug, Clone)]
enum Facing {
    Up,
    Down,
    North,
    South,
    East,
    West,
}
impl Facing {
    fn compliment(&self) -> Facing {
        match self {
            Facing::Up => Facing::Down,
            Facing::Down => Facing::Up,
            Facing::North => Facing::South,
            Facing::South => Facing::North,
            Facing::East => Facing::West,
            Facing::West => Facing::East,
        }
    }
}

#[derive(Debug, Clone)]
struct Cursor3D {
    pub position: Point3D,
    pub facing: Facing,
    pub cube_width: i32,
}
impl Cursor3D {
    fn new(x: i32, y: i32, z: i32, cube_width: i32) -> Self {
        Self {
            position: Point3D::new(x, y, z),
            facing: Facing::East,
            cube_width,
        }
    }
    fn which_face_am_i_on(&self) -> Option<Facing> {
        if self.position.z == -1 {
            Some(Facing::Up)
        } else if self.position.z == self.cube_width {
            Some(Facing::Down)
        } else if self.position.y == -1 {
            Some(Facing::North)
        } else if self.position.y == self.cube_width {
            Some(Facing::South)
        } else if self.position.x == -1 {
            Some(Facing::West)
        } else if self.position.x == self.cube_width {
            Some(Facing::East)
        } else {
            // println!("Cursor not on any face! {:?}", self.position);
            None
        }
    }
    fn where_is_next_step(&self) -> (Point3D, Facing) {
        let mut point_to_step_to = self.position.clone();
        let mut new_facing = self.facing.clone();
        match self.facing {
            Facing::Up => {
                point_to_step_to.z -= 1;
                if point_to_step_to.z == -1 {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::North => {
                            new_facing = Facing::South;
                            point_to_step_to.y += 1;
                        }
                        Facing::South => {
                            new_facing = Facing::North;
                            point_to_step_to.y -= 1;
                        }
                        Facing::East => {
                            new_facing = Facing::West;
                            point_to_step_to.x -= 1;
                        }
                        Facing::West => {
                            new_facing = Facing::East;
                            point_to_step_to.x += 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
            Facing::Down => {
                point_to_step_to.z += 1;
                if point_to_step_to.z == self.cube_width {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::North => {
                            new_facing = Facing::South;
                            point_to_step_to.y += 1;
                        }
                        Facing::South => {
                            new_facing = Facing::North;
                            point_to_step_to.y -= 1;
                        }
                        Facing::East => {
                            new_facing = Facing::West;
                            point_to_step_to.x -= 1;
                        }
                        Facing::West => {
                            new_facing = Facing::East;
                            point_to_step_to.x += 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
            Facing::North => {
                point_to_step_to.y -= 1;
                if point_to_step_to.y == -1 {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::Up => {
                            new_facing = Facing::Down;
                            point_to_step_to.z += 1;
                        }
                        Facing::Down => {
                            new_facing = Facing::Up;
                            point_to_step_to.z -= 1;
                        }
                        Facing::East => {
                            new_facing = Facing::West;
                            point_to_step_to.x -= 1;
                        }
                        Facing::West => {
                            new_facing = Facing::East;
                            point_to_step_to.x += 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
            Facing::South => {
                point_to_step_to.y += 1;
                if point_to_step_to.y == self.cube_width {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::Up => {
                            new_facing = Facing::Down;
                            point_to_step_to.z += 1;
                        }
                        Facing::Down => {
                            new_facing = Facing::Up;
                            point_to_step_to.z -= 1;
                        }
                        Facing::East => {
                            new_facing = Facing::West;
                            point_to_step_to.x -= 1;
                        }
                        Facing::West => {
                            new_facing = Facing::East;
                            point_to_step_to.x += 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
            Facing::East => {
                point_to_step_to.x += 1;
                if point_to_step_to.x == self.cube_width {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::Up => {
                            new_facing = Facing::Down;
                            point_to_step_to.z += 1;
                        }
                        Facing::Down => {
                            new_facing = Facing::Up;
                            point_to_step_to.z -= 1;
                        }
                        Facing::North => {
                            new_facing = Facing::South;
                            point_to_step_to.y += 1;
                        }
                        Facing::South => {
                            new_facing = Facing::North;
                            point_to_step_to.y -= 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
            Facing::West => {
                point_to_step_to.x -= 1;
                if point_to_step_to.x == -1 {
                    match self.which_face_am_i_on().unwrap() {
                        Facing::Up => {
                            new_facing = Facing::Down;
                            point_to_step_to.z += 1;
                        }
                        Facing::Down => {
                            new_facing = Facing::Up;
                            point_to_step_to.z -= 1;
                        }
                        Facing::North => {
                            new_facing = Facing::South;
                            point_to_step_to.y += 1;
                        }
                        Facing::South => {
                            new_facing = Facing::North;
                            point_to_step_to.y -= 1;
                        }
                        _ => {
                            panic!("Invalid movement");
                        }
                    }
                }
            }
        }
        (point_to_step_to, new_facing)
    }
    fn force_step(&mut self) {
        let next_step = self.where_is_next_step();
        // if next_step.0.y == 51 && next_step.0.y == 48 {
        // println!("Getting closer");
        //     // panic!("How did this happen?");
        // }
        // println!("Marching from {:?} to {:?}", self.position, next_step.0);
        // println!("Facing was {:?} and is now {:?}", self.facing, next_step.1);
        // print!(
        //     "I was on the {:?} side of the cube, ",
        //     self.which_face_am_i_on()
        // );
        self.position = next_step.0;
        self.facing = next_step.1;

        // println!(
        //     "Now I'm on the {:?} side of the cube.",
        //     self.which_face_am_i_on()
        // );
    }

    fn turn_around(&mut self) {
        // println!("turning around");
        self.facing = self.facing.compliment();
    }

    fn turn_cw(&mut self) {
        // println!("turning cw");
        match self.which_face_am_i_on().unwrap() {
            Facing::Up => match self.facing {
                Facing::North => self.facing = Facing::East,
                Facing::South => self.facing = Facing::West,
                Facing::East => self.facing = Facing::South,
                Facing::West => self.facing = Facing::North,
                _ => panic!("Can't turn due to invalid facing..."),
            },
            Facing::Down => match self.facing {
                Facing::North => self.facing = Facing::West,
                Facing::South => self.facing = Facing::East,
                Facing::East => self.facing = Facing::North,
                Facing::West => self.facing = Facing::South,
                _ => panic!("Can't turn due to invalid facing..."),
            },
            Facing::North => match self.facing {
                Facing::Up => self.facing = Facing::West,
                Facing::Down => self.facing = Facing::East,
                Facing::East => self.facing = Facing::Up,
                Facing::West => self.facing = Facing::Down,
                _ => panic!("Can't turn due to invalid facing..."),
            },
            Facing::South => match self.facing {
                Facing::Up => self.facing = Facing::East,
                Facing::Down => self.facing = Facing::West,
                Facing::East => self.facing = Facing::Down,
                Facing::West => self.facing = Facing::Up,
                _ => panic!("Can't turn due to invalid facing..."),
            },
            Facing::East => match self.facing {
                Facing::Up => self.facing = Facing::North,
                Facing::Down => self.facing = Facing::South,
                Facing::North => self.facing = Facing::Down,
                Facing::South => self.facing = Facing::Up,
                _ => panic!("Can't turn due to invalid facing..."),
            },
            Facing::West => match self.facing {
                Facing::Up => self.facing = Facing::South,
                Facing::Down => self.facing = Facing::North,
                Facing::North => self.facing = Facing::Up,
                Facing::South => self.facing = Facing::Down,
                _ => panic!("Can't turn due to invalid facing..."),
            },
        }
    }

    fn turn_ccw(&mut self) {
        // print!("turning ccw\t");
        self.turn_cw();
        self.turn_around();
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn as_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Game {
    flat_map: IndexMap<Point2D, Tile>,
    flat_x_bound: usize,
    flat_y_bound: usize,
    cube_map: IndexMap<Point3D, Point2D>,
    cube_width: usize,
    player: Cursor3D,
    directions: String,
}

fn input() -> Game {
    let raw = input_raw(DAY);
    let sections: Vec<String> = raw.split("\n\n").map(|item| item.to_owned()).collect();

    let map_raw = sections[0].clone();
    let map_raw_lines: Vec<String> = map_raw.split("\n").map(|item| item.to_owned()).collect();
    let directions = sections[1].clone();

    let mut flat_x_bound = 0usize;
    let flat_y_bound = map_raw_lines.len();
    for line in map_raw_lines.iter() {
        if line.len() > flat_x_bound {
            flat_x_bound = line.len();
        }
    }
    let larger_bound = max(flat_x_bound, flat_y_bound);
    let cube_width = larger_bound / 4;
    let mut map_padded_lines: Vec<String> = Vec::with_capacity(flat_y_bound);
    for line in map_raw_lines.iter() {
        map_padded_lines.push(line.pad_to_width_with_char(larger_bound, ' '));
    }
    while map_padded_lines.len() < larger_bound {
        map_padded_lines.push(" ".repeat(larger_bound));
    }

    let mut char_array_2d: IndexMap<Point2D, char> = IndexMap::new();

    for y in 0..larger_bound {
        let line_chars: Vec<char> = map_padded_lines[y].chars().collect();
        for x in 0..larger_bound {
            let point = Point2D::new(x as i32, y as i32);
            let char_to_insert = line_chars[x];
            char_array_2d.insert(point, char_to_insert);
        }
    }

    // find the first face
    // let mut first_face_x: Option<i32> = None;
    // for x in 0i32..4i32 {
    //     let point = Point2D::new(x * cube_width as i32, 0);
    //     if *char_array_2d.get(&point).unwrap() != ' ' {
    //         first_face_x = Some(x);
    //         break;
    //     }
    // }
    // assert_ne!(first_face_x, None);

    // identify the faces
    let mut first_face: Option<(i32, i32)> = None;
    let mut faces_grid = [[false; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            let point = Point2D::new(x * cube_width as i32, y * cube_width as i32);
            if *char_array_2d.get(&point).unwrap() != ' ' {
                if first_face.is_none() {
                    first_face = Some((x, y));
                }
                // let face_index = y as usize * 4 + x as usize;
                faces_grid[y as usize][x as usize] = true;
            }
        }
    }
    println!("FACES GRID:\n{:?}", faces_grid);
    assert_ne!(first_face, None);
    let first_face = first_face.unwrap();

    let mut cube_map: IndexMap<Point3D, Point2D> = IndexMap::new();
    let mut flat_map: IndexMap<Point2D, Tile> = IndexMap::new();

    let cursor_2d = Point2D::new(
        first_face.0 * cube_width as i32,
        first_face.1 * cube_width as i32,
    );
    let cursor_3d = Cursor3D::new(0, 0, -1, cube_width as i32);

    // let mut faces_to_be_mapped = faces_grid.clone();

    let face_cursor = first_face.clone();
    // faces_to_be_mapped[face_cursor.0 as usize][face_cursor.1 as usize] = false;
    // map_face(
    //     &mut cursor_2d,
    //     &mut cursor_3d,
    //     &mut char_array_2d,
    //     &mut flat_map,
    //     &mut cube_map,
    // );

    let mut checked = [[false; 4]; 4];
    let mut fringe: VecDeque<(Point2D, Cursor3D, (i32, i32))> = VecDeque::new();
    fringe.push_back((cursor_2d.clone(), cursor_3d.clone(), face_cursor));

    // let mut faces_mapped_or_skipped = 1;

    while !fringe.is_empty() {
        let (cursor_2d, cursor_3d, face_cursor) = fringe.pop_front().unwrap();
        map_face(
            &mut cursor_2d.clone(),
            &mut cursor_3d.clone(),
            &mut char_array_2d,
            &mut flat_map,
            &mut cube_map,
        );
        checked[face_cursor.1 as usize][face_cursor.0 as usize] = true;

        let neighbor_north = (face_cursor.0, face_cursor.1 - 1);
        if neighbor_north.1 >= 0
            && neighbor_north.1 <= 3
            && neighbor_north.0 >= 0
            && neighbor_north.0 <= 3
            && checked[neighbor_north.1 as usize][neighbor_north.0 as usize] == false
            && faces_grid[neighbor_north.1 as usize][neighbor_north.0 as usize] == true
        {
            println!("\nPREPPING FOR NORTH NEIGHBOR\n");
            // Clone the cursors
            let mut new_cursor_2d = cursor_2d.clone();
            let mut new_cursor_3d = cursor_3d.clone();
            // Move the cursors both north cube_width times
            new_cursor_2d.y -= cube_width as i32 - 1;
            new_cursor_3d.turn_ccw();
            for _ in 0..cube_width {
                new_cursor_3d.force_step();
            }
            new_cursor_3d.turn_cw();
            // Push the cursors to a fringe
            fringe.push_back((new_cursor_2d, new_cursor_3d, neighbor_north));
        }
        let neighbor_south = (face_cursor.0, face_cursor.1 + 1);
        // println!("Neighbor south: {:?}", neighbor_south);
        if neighbor_south.1 >= 0
            && neighbor_south.1 <= 3
            && neighbor_south.0 >= 0
            && neighbor_south.0 <= 3
            && checked[neighbor_south.1 as usize][neighbor_south.0 as usize] == false
            && faces_grid[neighbor_south.1 as usize][neighbor_south.0 as usize] == true
        {
            println!("\nPREPPING FOR SOUTH NEIGHBOR\n");
            // Clone the cursors
            let mut new_cursor_2d = cursor_2d.clone();
            let mut new_cursor_3d = cursor_3d.clone();
            // Move the cursors both south cube_width times
            println!("new_cursor_2d: {:?}", new_cursor_2d);
            new_cursor_2d.y += cube_width as i32;
            println!("new_cursor_2d: {:?}", new_cursor_2d);
            new_cursor_3d.turn_cw();
            for _ in 0..cube_width {
                new_cursor_3d.force_step();
            }
            new_cursor_3d.turn_ccw();
            // Push the cursors to a fringe
            fringe.push_back((new_cursor_2d, new_cursor_3d, neighbor_south));
        }
        let neighbor_east = (face_cursor.0 + 1, face_cursor.1);
        if neighbor_east.1 >= 0
            && neighbor_east.1 <= 3
            && neighbor_east.0 >= 0
            && neighbor_east.0 <= 3
            && checked[neighbor_east.1 as usize][neighbor_east.0 as usize] == false
            && faces_grid[neighbor_east.1 as usize][neighbor_east.0 as usize] == true
        {
            println!("\nPREPPING FOR EAST NEIGHBOR\n");
            // Clone the cursors
            let mut new_cursor_2d = cursor_2d.clone();
            let mut new_cursor_3d = cursor_3d.clone();
            // Move the cursors both east cube_width times
            new_cursor_2d.x += cube_width as i32;
            // new_cursor_3d.turn_cw();
            for _ in 0..cube_width {
                new_cursor_3d.force_step();
            }
            // new_cursor_3d.turn_ccw();
            // Push the cursors to a fringe
            fringe.push_back((new_cursor_2d, new_cursor_3d, neighbor_east));
        }
        let neighbor_west = (face_cursor.0 - 1, face_cursor.1);
        if neighbor_west.1 >= 0
            && neighbor_west.1 <= 3
            && neighbor_west.0 >= 0
            && neighbor_west.0 <= 3
            && checked[neighbor_west.1 as usize][neighbor_west.0 as usize] == false
            && faces_grid[neighbor_west.1 as usize][neighbor_west.0 as usize] == true
        {
            println!("\nPREPPING FOR WEST NEIGHBOR\n");
            // Clone the cursors
            let mut new_cursor_2d = cursor_2d.clone();
            let mut new_cursor_3d = cursor_3d.clone();
            // Move the cursors both west cube_width times
            new_cursor_2d.x -= cube_width as i32;
            new_cursor_3d.turn_around();
            for _ in 0..cube_width {
                new_cursor_3d.force_step();
            }
            new_cursor_3d.turn_around();
            // Push the cursors to a fringe
            fringe.push_back((new_cursor_2d, new_cursor_3d, neighbor_west));
        }
    }

    // let mut num_faces_mapped = 1;

    // println!("Face scan finished\n2D: {:?}", cursor_2d);
    // println!("3D: {:?}", cursor_3d);

    let player = Cursor3D::new(0, 0, -1, cube_width as i32);

    Game {
        flat_map,
        flat_x_bound,
        flat_y_bound,
        cube_map,
        cube_width,
        player,
        directions,
    }
}

fn map_face(
    cursor_2d: &mut Point2D,
    cursor_3d: &mut Cursor3D,
    char_array_2d: &mut IndexMap<Point2D, char>,
    flat_map: &mut IndexMap<Point2D, Tile>,
    cube_map: &mut IndexMap<Point3D, Point2D>,
) {
    println!("\n\n\nMAPPING A FACE\n\n\n");
    let cube_width = cursor_3d.cube_width;
    for y_offset in 0..cube_width {
        for _x_offset in 0..cube_width {
            println!("Scanning {:?}", cursor_2d);
            println!("Populating {:?}", cursor_3d);
            match char_array_2d.get(cursor_2d).unwrap() {
                '.' => {
                    flat_map.insert(cursor_2d.clone(), Tile::Path);
                }
                '#' => {
                    flat_map.insert(cursor_2d.clone(), Tile::Wall);
                }
                _ => {}
            }
            if cube_map.contains_key(&cursor_3d.position) {
                panic!("NO! {:?}", cursor_3d.position);
            }
            cube_map.insert(cursor_3d.position.clone(), cursor_2d.clone());

            cursor_2d.x += 1;
            cursor_3d.force_step();
        }

        // println!("Carriage return");
        cursor_2d.x -= cube_width as i32;
        cursor_3d.turn_around();
        for _ in 0..cube_width {
            cursor_3d.force_step();
        }
        cursor_3d.turn_around();

        // println!("y_offset: {}", y_offset);
        if y_offset != cube_width - 1 {
            // println!("New line feed");
            cursor_2d.y += 1;

            cursor_3d.turn_cw();
            cursor_3d.force_step();
            cursor_3d.turn_ccw();
        } else {
            // println!("Moving back up");
            cursor_2d.y -= cube_width;

            cursor_3d.turn_ccw();
            for _ in 0..cube_width - 1 {
                cursor_3d.force_step();
            }
            cursor_3d.turn_cw();
            // println!("Done");
        }
    }
    // println!("\n\n\nMAPPING DONE\n\n\n");
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
                game.player.turn_ccw();
                buffer = Vec::new();

                println!("Player: {:?}", game.player);
            }
            'R' => {
                let buffer_string: String = buffer.into_iter().collect();
                let steps_to_take = str::parse::<i32>(buffer_string.as_str()).unwrap();
                take_steps(game, steps_to_take);
                game.player.turn_cw();
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
        let (peek, _) = game.player.where_is_next_step();
        let peek_to_2d = game
            .cube_map
            .get(&peek)
            .expect("The cube map is acting up...");
        let tile = game
            .flat_map
            .get(peek_to_2d)
            .expect("The flat map is acting up...");
        if *tile == Tile::Wall {
            return;
        }
        game.player.force_step();
    }
}

pub fn d22s2(submit: bool) {
    let mut game = input();

    println!(
        "THE BIG DEBUG:\n{:?}\n\nlength:{}",
        game.cube_map,
        game.cube_map.len()
    );

    follow_path(&mut game);

    let player_3d_position = &game.player.position;
    let player_2d_position = game
        .cube_map
        .get(player_3d_position)
        .expect("Cube map doesn't contain the player...");

    let (player_3d_position_next_step, _) = game.player.where_is_next_step();
    let player_2d_position_next_step = game
        .cube_map
        .get(&player_3d_position_next_step)
        .expect("Cube map doesn't contain the player's next step...");

    // fn score(&self) -> i32 {
    //     match self {
    //         Facing::North => 3,
    //         Facing::East => 0,
    //         Facing::South => 1,
    //         Facing::West => 2,
    //     }
    // }

    let mut facing_score = 0;
    if player_2d_position_next_step.y > player_2d_position.y {
        facing_score = 1;
    } else if player_2d_position_next_step.y < player_2d_position.y {
        facing_score = 3;
    } else if player_2d_position_next_step.x > player_2d_position.x {
        facing_score = 0;
    } else if player_2d_position_next_step.x < player_2d_position.x {
        facing_score = 2;
    }

    let answer = 1000 * (player_2d_position.y + 1) + 4 * (player_2d_position.x + 1) + facing_score;

    final_answer(answer, submit, DAY, 2);
}
