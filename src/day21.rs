use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[derive(PartialEq)]
enum Opcode {
    Add,
    Mul,
    Halt,
    Illegal,
}

impl Into<Opcode> for usize {

    fn into(self) -> Opcode {
        match self {
            1 => Add,
            2 => Mul,
            99 => Halt,
            _ => Illegal
        }
    }

}

pub fn part1() -> usize {
    let opcodes = parse_input(INPUT_2019_2);
    if opcodes.is_empty() { panic!(); }

    run_computer(opcodes, 12, 2)

}

fn run_computer(opcodes: Vec<usize>, noun: usize, verb: usize) -> usize {

    let mut opcodes = opcodes;

    opcodes[1] = noun;
    opcodes[2] = verb;

    let mut i = 0;
    loop {

        let opcode: Opcode = opcodes[i].into();

        if opcode == Illegal { panic!(); }
        if opcode == Halt { break; }

        //else

        let op1 = opcodes[opcodes[i+1]];
        let op2 = opcodes[opcodes[i+2]];
        let dest_index = opcodes[i+3];

        if opcode == Add { opcodes[dest_index] = op1 + op2 }
        else { opcodes[dest_index] = op1 * op2; }

        i += 4; //advance to next opcode
    }

    opcodes[0]
}

pub fn part2() -> usize {
    let opcodes = parse_input(INPUT_2019_2);

    const PUZZLE_OUTPUT: usize = 19690720;

    if opcodes.is_empty() { panic!(); }

    for verb in 0..=99 {
        for noun in 0..=99 {
            if run_computer(opcodes.clone(), noun, verb) == PUZZLE_OUTPUT {
                return (noun * 100) + verb;
            }
        }
    }

    panic!("Not found");
}