use std::collections::VecDeque;

use indexmap::IndexMap;

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
    fn as_tuple(&self) -> (u8, u8, u8) {
        (self.x, self.y, self.z)
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
            str::parse::<u8>(coord_components.next().unwrap()).unwrap() + 1,
            str::parse::<u8>(coord_components.next().unwrap()).unwrap() + 1,
            str::parse::<u8>(coord_components.next().unwrap()).unwrap() + 1,
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

#[derive(PartialEq)]
enum Material {
    Air,
    Cube,
    Void,
}

pub fn d18s2(submit: bool) {
    let cubes = input();
    // let _number_of_cubes = cubes.len();
    // let mut _number_of_adjacencies = 0usize;
    // let mut _cubes_checked: Vec<&Cube> = Vec::with_capacity(cubes.len());
    let mut x_max = cubes[0].x;
    let mut y_max = cubes[0].y;
    let mut z_max = cubes[0].z;
    for cube in cubes.iter() {
        if cube.x > x_max {
            x_max = cube.x;
        }
        if cube.y > y_max {
            y_max = cube.y;
        }
        if cube.z > z_max {
            z_max = cube.z;
        }
    }
    x_max += 1;
    y_max += 1;
    z_max += 1;

    let mut map: IndexMap<(u8, u8, u8), Material> = IndexMap::new();
    for x in 0..=x_max {
        for y in 0..=y_max {
            for z in 0..=z_max {
                map.insert((x, y, z), Material::Void);
            }
        }
    }
    for cube in cubes.iter() {
        map.insert(cube.as_tuple(), Material::Cube);
    }

    let mut air_fringe: VecDeque<Cube> = VecDeque::new();
    air_fringe.push_back(Cube::new(0, 0, 0));
    while air_fringe.len() > 0 {
        let current = air_fringe.pop_front().unwrap();
        let current_tuple = current.as_tuple();
        let map_at_current = map
            .get(&current_tuple)
            .expect(format!("No value at {:?}", current_tuple).as_str());
        if *map_at_current != Material::Void {
            continue;
        }
        map.insert(current_tuple, Material::Air);
        let neighbors = [
            current.cube_above(),
            current.cube_below(),
            current.cube_north(),
            current.cube_south(),
            current.cube_east(),
            current.cube_west(),
        ];
        for neighbor in neighbors {
            if neighbor.x <= x_max && neighbor.y <= y_max && neighbor.z <= z_max {
                air_fringe.push_back(neighbor);
            }
        }
    }

    let mut external_surface_area = 0usize;
    for current in cubes.iter() {
        let neighbors = [
            current.cube_above(),
            current.cube_below(),
            current.cube_north(),
            current.cube_south(),
            current.cube_east(),
            current.cube_west(),
        ];
        for neighbor in neighbors {
            let neighbor_tuple = neighbor.as_tuple();
            let material_at_neighbor_location = map
                .get(&neighbor_tuple)
                .expect(format!("No value at {:?}", neighbor_tuple).as_str());
            match material_at_neighbor_location {
                Material::Air => {
                    external_surface_area += 1;
                }
                _ => {}
            }
        }
    }
    final_answer(external_surface_area, submit, DAY, 1);
}
