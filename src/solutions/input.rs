use std::fs;

pub fn get_lines(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path)
        .expect("failed to load input");

    data.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

pub fn get_data<T>(path: &str, parse_fn: impl Fn(&str) -> T) -> Vec<T> {
    let data = fs::read_to_string(path)
        .expect("failed to load input");

    data.lines().map(|s| parse_fn(s)).collect::<Vec<T>>()
}