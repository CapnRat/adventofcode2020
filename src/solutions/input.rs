use std::fs;

pub fn get_lines(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path)
        .expect("failed to load input");

    data.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}