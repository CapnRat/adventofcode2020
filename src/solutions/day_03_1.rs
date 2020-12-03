use crate::solutions::input::get_chars;
use std::collections::HashSet;

pub fn run() -> String {
    let chars = get_chars("input/input_03");

    let mut grid = HashSet::new();
    let mut grid_x = 0;
    let mut grid_y = 0;
    for char in chars.into_iter() {
        match char {
            '.' => grid_x += 1,
            '#' => {
                grid.insert((grid_x, grid_y));
                grid_x += 1;
            },
            '\n' => {
                grid_x = 0;
                grid_y += 1;
            },
            '\r' => {}
            _ => panic!("Unexpected character in input")
        }
    }

    let mut trees = 0;
    let mut pos = (0, 0);
    while pos.1 <= grid_y {
        if grid.contains(&pos) { trees += 1 }
        pos.0 = (pos.0 + 3) % grid_x;
        pos.1 = pos.1 + 1;
    }

    trees.to_string()
}