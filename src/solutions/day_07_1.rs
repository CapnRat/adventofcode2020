use crate::solutions::input::get_lines;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub struct Bag {
    pub adjective: String,
    pub color: String
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Rule {
    pub count: u32,
    pub bag: Bag
}

pub fn run() -> String {
    let mut bag_rule_map: HashMap<Bag, Vec<Rule>> = HashMap::new();

    get_lines("input/input_07").into_iter()
        .for_each(|line| {
            let splits = line.split(" contain ").collect_vec();
            let (adjective, color, _) = scan_fmt!(splits[0], "{} {} {}", String, String, String).unwrap();
            bag_rule_map.insert(Bag {adjective, color}, splits[1].split(",").filter(|&s| s != "no other bags.")
                .map(|rule_raw| {
                    let (count, adjective, color, _) = scan_fmt!(rule_raw, "{d} {} {} {}", u32, String, String, String).unwrap();
                    Rule {count, bag: Bag { adjective, color }}
                })
                .collect_vec());
        });

    let target_bag = Bag { adjective: String::from("shiny"), color: String::from("gold") };
    bag_rule_map.iter().filter(|&b| contains_bag(&bag_rule_map, b.0, &target_bag)).count().to_string()
}

fn contains_bag (map: &HashMap<Bag, Vec<Rule>>, bag: &Bag, contains: &Bag) -> bool {
    for rule in &map[bag] {
        if rule.bag == *contains || contains_bag(&map, &rule.bag, contains) {
            return true;
        }
    }
    false
}