use crate::solutions::input::get_lines;

pub fn run() -> String {
    let expenses: Vec<i32> = get_lines("input/input_01").iter().map(|l| l.parse::<i32>().unwrap()).collect();

    for (i, expense) in expenses.iter().enumerate() {
        for j in i+1..expenses.len() {
            if expense + expenses[j] == 2020 {
                return (expense * expenses[j]).to_string()
            }
        }
    }

    panic!("No solution found!");
}