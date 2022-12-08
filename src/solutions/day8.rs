use super::final_answer;
use super::input_raw;

const DAY: u8 = 8;

fn x_and_y_to_index(x: usize, y: usize, cols: usize) -> usize {
    x * cols + y
}

struct TreeMap {
    heights: Vec<u8>,
    visible: Vec<bool>,
    rows: usize,
    cols: usize,
}
impl TreeMap {
    pub fn set_height(&mut self, x: usize, y: usize, height: u8) {
        self.heights[x_and_y_to_index(x, y, self.cols)] = height;
    }
    pub fn get_height(&mut self, x: usize, y: usize) -> u8 {
        self.heights[x_and_y_to_index(x, y, self.cols)]
    }
    pub fn set_visible(&mut self, x: usize, y: usize, visible: bool) {
        self.visible[x_and_y_to_index(x, y, self.cols)] = visible;
    }
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            heights: vec![0u8; rows * cols],
            visible: vec![false; rows * cols],
            rows,
            cols,
        }
    }
}

fn input() -> TreeMap {
    let input_raw = input_raw(DAY);
    let input: Vec<&str> = input_raw.split("\n").collect();
    let rows = input.len();
    let cols = input[0].len();
    let mut tree_map = TreeMap::new(rows, cols);
    let mut x = 0usize;
    let zero = '0' as u8;
    for line in input {
        let chars = line.as_bytes().to_vec();
        for y in 0..cols {
            tree_map.set_height(x, y, chars[y] - zero);
            tree_map.set_visible(x, y, x == 0 || x == (cols - 1) || y == 0 || y == (rows - 1));
        }
        x += 1;
    }

    tree_map
}

pub fn d8s1(submit: bool) {
    let mut tree_map = input();

    // horizontal
    for y in 1usize..(tree_map.rows - 1) {
        let mut x_max_from_left = tree_map.get_height(0, y);
        let mut x_max_from_right = tree_map.get_height(tree_map.cols - 1, y);
        // left
        for x in 1usize..(tree_map.cols - 1) {
            let test = tree_map.get_height(x, y);
            if test > x_max_from_left {
                x_max_from_left = test;
                tree_map.set_visible(x, y, true);
                if test >= 9 {
                    break;
                }
            }
        }
        // right
        for x in 1usize..(tree_map.cols - 1) {
            let test = tree_map.get_height((tree_map.cols - 1) - x, y);
            if test > x_max_from_right {
                x_max_from_right = test;
                tree_map.set_visible((tree_map.cols - 1) - x, y, true);
                if test >= 9 {
                    break;
                }
            }
        }
    }

    // vertical
    for x in 1usize..tree_map.cols - 1 {
        let mut y_max_from_top = tree_map.get_height(x, 0);
        let mut y_max_from_bot = tree_map.get_height(x, tree_map.rows - 1);
        //top
        for y in 1usize..(tree_map.rows - 1) {
            let test = tree_map.get_height(x, y);
            if test > y_max_from_top {
                y_max_from_top = test;
                tree_map.set_visible(x, y, true);
                if test >= 9 {
                    break;
                }
            }
        }
        //bot
        for y in 1usize..(tree_map.rows - 1) {
            let test = tree_map.get_height(x, (tree_map.rows - 1) - y);
            if test > y_max_from_bot {
                y_max_from_bot = test;
                tree_map.set_visible(x, (tree_map.rows - 1) - y, true);
                if test >= 9 {
                    break;
                }
            }
        }
    }

    let mut count = 0usize;
    for visible_test in tree_map.visible {
        if visible_test {
            count += 1;
        }
    }

    final_answer(count, submit, DAY, 1);
}

pub fn d8s2(submit: bool) {
    let mut tree_map = input();

    let mut best_score = 0u64;

    for x in 1..(tree_map.cols - 1) {
        for y in 1..(tree_map.rows - 1) {
            let tree_house_height = tree_map.get_height(x, y);
            // left
            let mut left_sight = 1usize;
            let mut left_score = 0u64;
            loop {
                left_score += 1;
                tree_map.set_visible(x - left_sight, y, true);
                if tree_map.get_height(x - left_sight, y) >= tree_house_height {
                    break;
                }
                if x - left_sight == 0 {
                    break;
                }
                left_sight += 1;
            }
            // right
            let mut right_sight = 1usize;
            let mut right_score = 0u64;
            while (x + right_sight) < tree_map.cols {
                right_score += 1;
                tree_map.set_visible(x + right_sight, y, true);
                if tree_map.get_height(x + right_sight, y) >= tree_house_height {
                    break;
                }
                right_sight += 1;
            }
            // up
            let mut up_sight = 1usize;
            let mut up_score = 0u64;
            loop {
                up_score += 1;
                tree_map.set_visible(x, y - up_sight, true);
                if tree_map.get_height(x, y - up_sight) >= tree_house_height {
                    break;
                }
                if y - up_sight == 0 {
                    break;
                }
                up_sight += 1;
            }
            // down
            let mut down_sight = 1usize;
            let mut down_score = 0u64;
            while (y + down_sight) < tree_map.rows {
                down_score += 1;
                tree_map.set_visible(x, y + down_sight, true);
                if tree_map.get_height(x, y + down_sight) >= tree_house_height {
                    break;
                }
                down_sight += 1;
            }
            let score = left_score * right_score * up_score * down_score;
            if score > best_score {
                best_score = score;
            }
        }
    }

    final_answer(best_score, submit, DAY, 2);
}
