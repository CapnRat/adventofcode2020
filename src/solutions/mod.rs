mod day_01_1;
mod input;

#[derive(Debug, Default)]
pub struct Solution {
    pub day: i8,
    pub star: i8
}

pub fn run_solution(solution: Solution) -> String {
    println!("Running solution {:?}", solution);
    match solution {
        Solution { day:  1, star: 1 } => day_01_1::run(),
        _ => String::from("Unknown solution")
    }
}

pub fn get_latest() -> Solution {
    Solution { day: 1, star: 1 }
}