use crate::solutions::input::get_data;
use crate::solutions::day_05_1::{BoardingPass, parse_bsp};

pub fn run() -> String {
    let mut passes: Vec<BoardingPass> = get_data("input/input_05", parse_bsp);
    passes.sort_by_key(|bp| bp.get_seat_id());

    let mut last_id = passes[0].get_seat_id() - 1;
    for pass in passes {
        if pass.get_seat_id() != last_id + 1 {
            return (last_id + 1).to_string()
        }
        last_id += 1
    }

    panic!("seat not found")
}