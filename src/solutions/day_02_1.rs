use crate::solutions::input::get_data;
use regex::Regex;

#[derive(Debug, Default, Eq, PartialEq)]
struct Policy {
    min: u32,
    max: u32,
    char: char
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Password {
    policy: Policy,
    value: String
}

pub fn run() -> String {
    let passwords: Vec<Password> = get_data("input/input_02", parse_line);

    let mut num_valid = 0;
    for password in passwords.into_iter() {
        let mut policy_char_count = 0;
        for c in password.value.chars() {
            if c == password.policy.char {
                policy_char_count += 1;
            }
        }
        if policy_char_count >= password.policy.min && policy_char_count <= password.policy.max {
            num_valid += 1;
        }
    }

    num_valid.to_string()
}

fn parse_line(line: &str) -> Password {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    Password {
        policy: Policy {
            min: (&cap[1]).to_string().parse::<u32>().unwrap(),
            max: (&cap[2]).to_string().parse::<u32>().unwrap(),
            char: (&cap[3]).to_string().chars().nth(0).unwrap()
        },
        value: (&cap[4]).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::{parse_line, Policy, Password};

    #[test]
    fn test_can_parse_password()
    {
        let expected = Password {
            policy: Policy {
                min: 1,
                max: 3,
                char: 'a'
            },
            value: "abcde".to_string()
        };

        let line = "1-3 a: abcde";

        let result = parse_line(line);

        assert_eq!(expected, result)
    }
}