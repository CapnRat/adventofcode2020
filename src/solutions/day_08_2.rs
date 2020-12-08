use crate::solutions::input::get_data;
use crate::solutions::day_08_1::{parse_instruction, Instruction, Op, run_program};

pub fn run() -> String {
    let instructions: Vec<Instruction> = get_data("input/input_08", parse_instruction);

    let mut accum = 0;
    for i in 0..instructions.len() {
        let mut candidate = instructions.clone();
        candidate[i].op = match candidate[i].op {
            Op::Acc => Op::Acc,
            Op::Jmp => Op::Nop,
            Op::Nop => Op::Jmp
        };

        let (result, loop_detected) = run_program(&candidate);
        if !loop_detected {
            accum = result;
            break
        }
    }

    accum.to_string()
}