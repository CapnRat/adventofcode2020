use crate::solutions::input::get_lines;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PassportData {
    pub raw_data: HashMap<String, String>
}

pub fn run() -> String {
    let data = get_lines("input/input_04");

    return parse_data(data).into_iter().filter(|p| is_valid(p)).count().to_string()
}

pub fn is_valid(passport_data: &PassportData) -> bool {
    if passport_data.raw_data.len() < 7 || passport_data.raw_data.len() > 8 {
        return false;
    }

    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .filter(|&f| !passport_data.raw_data.contains_key(&f.to_string())).count() == 0
}

pub fn parse_data(data: Vec<String>) -> Vec<PassportData> {
    let mut passports: Vec<PassportData> = Vec::new();
    passports.push(PassportData { raw_data: HashMap::new() });

    for line in data.into_iter() {
        if line.is_empty() {
            passports.push(PassportData { raw_data: HashMap::new() });
            continue
        }
        line.split(" ")
            .map(|s| scan_fmt!(s, "{}:{}", String, String).unwrap())
            .for_each(|t| {
                passports.last_mut().unwrap().raw_data.insert(t.0, t.1);
            });
    }
    passports
}