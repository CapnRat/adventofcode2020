use crate::solutions::input::get_chars;
use crate::solutions::day_03_1::{load_grid, count_trees};

pub fn run() -> String {
    let (grid, grid_width, grid_height) = load_grid(get_chars("input/input_03"));

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|s| count_trees(&grid, grid_width, grid_height, s.clone()) as i64).product::<i64>().to_string()
}