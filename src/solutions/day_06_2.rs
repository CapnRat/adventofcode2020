use crate::solutions::input::get_data_string;
use itertools::Itertools;

pub fn run() -> String {
    get_data_string("input/input_06")
        .split("\n\n")
        .map(|group| group.chars().filter(|&c| c != '\n')
            .unique()
            .map(|answer| group.chars().filter(|&c| c == answer).count())
            .filter(|&count| count == group.lines().count())
            .count()
        ).sum::<usize>()
        .to_string()
}