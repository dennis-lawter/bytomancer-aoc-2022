use std::collections::VecDeque;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 18;

struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Cube {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
    fn cube_above(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }
    fn cube_below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
    fn cube_north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }
    fn cube_south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }
    fn cube_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }
    fn cube_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }
    fn is_adjacent(&self, other: &Self) -> bool {
        *other == self.cube_above()
            || *other == self.cube_below()
            || *other == self.cube_north()
            || *other == self.cube_south()
            || *other == self.cube_east()
            || *other == self.cube_west()
    }
}
impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Clone for Cube {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

fn input() -> Vec<Cube> {
    let raw = input_raw(DAY);
    let lines: Vec<&str> = raw.split("\n").collect();
    let mut result: Vec<Cube> = Vec::with_capacity(lines.len());
    for line in lines {
        let mut coord_components = line.split(",");
        result.push(Cube::new(
            str::parse::<u8>(coord_components.next().unwrap()).unwrap(),
            str::parse::<u8>(coord_components.next().unwrap()).unwrap(),
            str::parse::<u8>(coord_components.next().unwrap()).unwrap(),
        ));
    }

    result
}

pub fn d18s1(submit: bool) {
    let cubes = input();
    let number_of_cubes = cubes.len();
    let mut number_of_adjacencies = 0usize;
    let mut cubes_checked: Vec<&Cube> = Vec::with_capacity(cubes.len());
    for cube in cubes.iter() {
        cubes_checked.push(cube);
        for other in cubes.iter() {
            if cubes_checked.contains(&other) {
                continue;
            }
            if cube.is_adjacent(other) {
                number_of_adjacencies += 1;
            }
        }
    }
    let surface_area = (number_of_cubes * 6) - (number_of_adjacencies * 2);
    final_answer(surface_area, submit, DAY, 1);
}

enum Material {
    Air,
    Cube,
    Void,
}

pub fn d18s2(submit: bool) {
    let cubes = input();
    let number_of_cubes = cubes.len();
    let mut number_of_adjacencies = 0usize;
    let mut cubes_checked: Vec<&Cube> = Vec::with_capacity(cubes.len());
    // let mut x_min = cubes[0].x;
    let mut x_max = cubes[0].x;
    // let mut y_min = cubes[0].y;
    let mut y_max = cubes[0].y;
    // let mut z_min = cubes[0].z;
    let mut z_max = cubes[0].z;
    for cube in cubes.iter() {
        if cube.x < x_min {
            x_min = cube.x;
        } else if cube.x > x_min {
            x_max = cube.x;
        }
        if cube.y < y_min {
            y_min = cube.y;
        } else if cube.y > y_min {
            y_max = cube.y;
        }
        if cube.z < z_min {
            z_min = cube.z;
        } else if cube.z > z_min {
            z_max = cube.z;
        }
    }
    // x_min -= 1;
    x_max += 1;
    // y_min -= 1;
    y_max += 1;
    // z_min -= 1;
    z_max += 1;
    // let x_range_magnitude = x_max - x_min;
    // let x_range_magnitude = x_max - x_min;
    // let x_range_magnitude = x_max - x_min;
    let total_surface_area = (number_of_cubes * 6) - (number_of_adjacencies * 2);

    let mut air_cubes_not_trapped: Vec<Cube> = Vec::new();
    let mut air_fringe: VecDeque<Cube> = VecDeque::new();
    air_fringe.push_back(Cube::new(x_min, y_min, z_min));
    'fringe_process: while air_fringe.len() > 0 {
        let current = air_fringe.pop_front().unwrap();
        for cube in cubes.iter() {
            if *cube == current {
                continue 'fringe_process;
            }
        }
        air_cubes_not_trapped.push(current.clone());
        let neighbors = [
            current.cube_above(),
            current.cube_below(),
            current.cube_north(),
            current.cube_south(),
            current.cube_east(),
            current.cube_west(),
        ];
        for neighbor in neighbors {
            if neighbor.x >= x_min
                && neighbor.x <= x_max
                && neighbor.y >= y_min
                && neighbor.y <= y_max
                && neighbor.z >= z_min
                && neighbor.z <= z_max
            {
                air_fringe.push_back(neighbor);
            }
        }
    }
    // for x in (x_min-1..(x_max+2) {
    //     for y in (y_min-1..y_max+2) {
    //         for z in (z_min..z_max+2) {
    //             let new_cube = Cube::new(x, y, z);
    //             if cubes.contains(x)
    //         }
    //     }
    // }
    // final_answer(surface_area, submit, DAY, 2);
}
