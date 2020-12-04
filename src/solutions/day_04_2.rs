use crate::solutions::input::get_lines;
use crate::solutions::day_04_1;
use crate::solutions::day_04_1::{parse_data, PassportData};

pub fn run() -> String {
    let data = get_lines("input/input_04");

    return parse_data(data).into_iter().filter(|p| is_valid(p)).count().to_string()
}

fn is_valid(passport_data: &PassportData) -> bool {
    passport_data.raw_data.iter().filter(|&p| {
        match p.0.as_str() {
            "byr" => !(1920..2003).contains(&(p.1.parse::<i32>().unwrap_or(0))),
            "iyr" => !(2010..2021).contains(&(p.1.parse::<i32>().unwrap_or(0))),
            "eyr" => !(2020..2031).contains(&(p.1.parse::<i32>().unwrap_or(0))),
            "hgt" => {
                if p.1.ends_with("cm") { let cm = scan_fmt!(p.1, "{d}", i32).unwrap_or(0); !(150..194).contains(&cm) }
                else if p.1.ends_with("in") { let inch = scan_fmt!(p.1, "{d}", i32).unwrap_or(0); !(59..77).contains(&inch) }
                else { true }
            },
            "hcl" => scan_fmt!(p.1, "#{[0-9a-f]}", String).is_err(),
            "ecl" => !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&p.1.as_str()),
            "pid" => scan_fmt!(p.1, "{d}", u32).is_err() || p.1.len() != 9,
            "cid" => false,
            _ => panic!("unexpected key")
        }
    }).count() == 0 && day_04_1::is_valid(passport_data)
}