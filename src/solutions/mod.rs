mod day_01_1;
mod day_01_2;
mod day_02_1;
mod day_02_2;
mod day_03_1;
mod day_03_2;
mod day_04_1;
mod day_04_2;
mod day_05_1;
mod day_05_2;
mod day_06_1;
mod day_06_2;
mod day_07_1;
mod day_07_2;
mod day_08_1;
mod day_08_2;
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
        Solution { day:  1, star: 2 } => day_01_2::run(),
        Solution { day:  2, star: 1 } => day_02_1::run(),
        Solution { day:  2, star: 2 } => day_02_2::run(),
        Solution { day:  3, star: 1 } => day_03_1::run(),
        Solution { day:  3, star: 2 } => day_03_2::run(),
        Solution { day:  4, star: 1 } => day_04_1::run(),
        Solution { day:  4, star: 2 } => day_04_2::run(),
        Solution { day:  5, star: 1 } => day_05_1::run(),
        Solution { day:  5, star: 2 } => day_05_2::run(),
        Solution { day:  6, star: 1 } => day_06_1::run(),
        Solution { day:  6, star: 2 } => day_06_2::run(),
        Solution { day:  7, star: 1 } => day_07_1::run(),
        Solution { day:  7, star: 2 } => day_07_2::run(),
        Solution { day:  8, star: 1 } => day_08_1::run(),
        Solution { day:  8, star: 2 } => day_08_2::run(),
        _ => String::from("Unknown solution")
    }
}

pub fn get_latest() -> Solution {
    Solution { day: 8, star: 2 }
}