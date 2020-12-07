use crate::solutions::day_07_1::{get_map, Bag, Rule};
use std::collections::HashMap;

pub fn run() -> String {
    let target_bag = Bag { adjective: String::from("shiny"), color: String::from("gold") };
    (count_bags(&get_map("input/input_07"), &target_bag, 1) - 1).to_string()
}

pub fn count_bags(map: &HashMap<Bag, Vec<Rule>>, bag: &Bag, multiplier: u32) -> u32 {
    let mut count = 1;
    for rule in &map[bag] {
        count += count_bags(map, &rule.bag, rule.count)
    }
    count * multiplier
}