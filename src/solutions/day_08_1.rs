use crate::solutions::input::get_data;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    Nop,
    Acc,
    Jmp
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub op: Op,
    pub val: i32
}

pub fn run() -> String {
    let instructions: Vec<Instruction> = get_data("input/input_08", parse_instruction);

    let (accum, _) = run_program(&instructions);

    accum.to_string()
}

pub fn run_program(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut visited: HashSet<usize> = HashSet::new();

    let mut accum = 0;
    let mut int_ptr = 0;
    let loop_detected = loop {
        let instruction = &instructions[int_ptr];

        if visited.contains(&int_ptr) {
            break true
        }

        visited.insert(int_ptr);

        match instruction.op {
            Op::Nop => int_ptr += 1,
            Op::Acc => {
                accum += instruction.val;
                int_ptr += 1;
            },
            Op::Jmp => int_ptr = ((int_ptr as i32) + instruction.val) as usize,
        }

        if int_ptr == instructions.len() {
            break false
        }
    };
    (accum, loop_detected)
}

pub fn parse_instruction(line: &str) -> Instruction {
    let (op_str, val) = scan_fmt!(line, "{} {d}", String, i32).unwrap();
    let op: Op = match op_str.as_str() {
        "nop" => Op::Nop,
        "acc" => Op::Acc,
        "jmp" => Op::Jmp,
        _ => panic!("unexpected op string {}", op_str)
    };

    Instruction {op, val}
}