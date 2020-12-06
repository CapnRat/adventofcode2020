use crate::solutions::input::get_data_string;
use itertools::Itertools;

pub fn run() -> String {
    get_data_string("input/input_06")
        .split("\n\n")
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).unique().count())
        .sum::<usize>()
        .to_string()
}