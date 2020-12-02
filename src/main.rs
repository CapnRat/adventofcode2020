#[macro_use] extern crate lazy_static;

mod solutions;

use clap::{Arg, App};
use solutions::Solution;
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let matches = App::new("Advent of Code 2020")
        .author("Shawn White <shawn@capnrat.com>")
        .arg(Arg::new("latest")
            .short('l')
            .long("latest"))
        .get_matches();

    let latest = matches.is_present("latest");

    let solution: Solution;
    if latest {
        solution = solutions::get_latest();
    } else {
        solution = Solution { day: read_value("Day"), star: read_value("Star") };
    }

    let start = Instant::now();
    let result = solutions::run_solution(solution);
    let duration = start.elapsed();

    println!("{}", result);
    println!("Finished after {:?}", duration);
}

fn read_value<T: FromStr>(name: &str) -> T {
    let mut value = String::new();
    print!("Enter {}: ", name);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut value).expect(&format!("Failed to read {}", name));

    match value.trim().parse::<T>() {
        Ok(val) => val,
        Err(_) => panic!("Failed to parse {}", name)
    }
}
