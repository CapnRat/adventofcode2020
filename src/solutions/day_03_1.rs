use crate::solutions::input::get_chars;
use std::collections::HashSet;

pub fn run() -> String {
    let chars = get_chars("input/input_03");

    let (grid, grid_x, grid_y) = load_grid(chars);

    let slope = (3, 1);
    let trees = count_trees(&grid, grid_x, grid_y, slope);

    trees.to_string()
}

pub fn count_trees(grid: &HashSet<(i32, i32)>, grid_width: i32, grid_height: i32, slope: (i32, i32)) -> i32 {
    let mut trees = 0;
    let mut pos = (0, 0);
    while pos.1 <= grid_height {
        if grid.contains(&pos) { trees += 1 }
        pos.0 = (pos.0 + slope.0) % grid_width;
        pos.1 = pos.1 + slope.1;
    }
    trees
}

pub fn load_grid(chars: Vec<char>) -> (HashSet<(i32, i32)>, i32, i32) {
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
            _ => panic!("Unexpected character in input")
        }
    }
    (grid, grid_x, grid_y)
}