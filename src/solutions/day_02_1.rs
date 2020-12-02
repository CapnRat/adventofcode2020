use crate::solutions::input::get_data;
use regex::Regex;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Policy {
    pub first: u32,
    pub second: u32,
    pub char: char
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Password {
    pub policy: Policy,
    pub value: String
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
        if policy_char_count >= password.policy.first && policy_char_count <= password.policy.second {
            num_valid += 1;
        }
    }

    num_valid.to_string()
}

pub fn parse_line(line: &str) -> Password {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    Password {
        policy: Policy {
            first: (&cap[1]).to_string().parse::<u32>().unwrap(),
            second: (&cap[2]).to_string().parse::<u32>().unwrap(),
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
                first: 1,
                second: 3,
                char: 'a'
            },
            value: "abcde".to_string()
        };

        let line = "1-3 a: abcde";

        let result = parse_line(line);

        assert_eq!(expected, result)
    }
}