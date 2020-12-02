use crate::solutions::input::get_data;
use crate::solutions::day_02_1::{parse_line, Password};

pub fn run() -> String {
    let passwords: Vec<Password> = get_data("input/input_02", parse_line);

    let mut num_valid = 0;
    for password in passwords.into_iter() {
        let first_char = password.value.chars().nth((password.policy.first - 1) as usize).unwrap();
        let second_char = password.value.chars().nth((password.policy.second - 1) as usize).unwrap();
        if (first_char == password.policy.char || second_char == password.policy.char) &&
            first_char != second_char {
            num_valid += 1;
        }
    }

    num_valid.to_string()
}