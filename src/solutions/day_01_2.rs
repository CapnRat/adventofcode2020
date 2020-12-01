use crate::solutions::input::get_lines;
//use itertools::Itertools;

pub fn run() -> String {
    let expenses: Vec<i32> = get_lines("input/input_01").iter().map(|l| l.parse::<i32>().unwrap()).collect();

/*    let mut result = 0;
    expenses.iter().copied().combinations(3).for_each(|c| if c.iter().sum::<i32>() == 2020 { result = c.iter().product() });
    return result.to_string();*/

    for (i, expense) in expenses.iter().enumerate() {
        for j in i+1..expenses.len() {
            if expense + expenses[j] < 2020 {
                for k in 0..expenses.len() {
                    if expense + expenses[j] + expenses[k] == 2020 {
                        return (expense * expenses[j] * expenses[k]).to_string()
                    }
                }
            }
        }
    }

    panic!("No solution found!");
}