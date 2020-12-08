use crate::solutions::input::get_data;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Nop,
    Acc,
    Jmp
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    pub op: Op,
    pub val: i32
}

pub fn run() -> String {
    let instructions: Vec<Instruction> = get_data("input/input_08", parse_instruction);
    let mut visited: HashSet<usize> = HashSet::new();

    let mut accum = 0;
    let mut int_ptr = 0;
    loop {
        let instruction = &instructions[int_ptr];

        if visited.contains(&int_ptr) {
            break
        }

        visited.insert(int_ptr);

        match instruction.op {
            Op::Nop => int_ptr += 1,
            Op::Acc => { accum += instruction.val; int_ptr += 1; },
            Op::Jmp => int_ptr = ((int_ptr as i32) + instruction.val) as usize,
        }
    }
    accum.to_string()
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
    //let val = val_str.parse::<u32>().unwrap();
}