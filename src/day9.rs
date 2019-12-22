use aoc_runner_derive::{aoc, aoc_generator};
use crate::opcode::{OpcodeRunner, Opcode};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(mem: &[isize]) -> isize {

    let test_mode = 1;
    let mut runner = OpcodeRunner::new(mem);

    let mut outputs: Vec<isize> = Vec::new();

    runner.push_input(test_mode);

    while !runner.has_halted() {
        let cur_opcode = runner.parse_cur_opcode();
        let got_output = runner.exec_opcode(cur_opcode);
        if got_output {
            outputs.push(runner.output().unwrap());
        }
    }


//    dbg!(outputs);
    outputs.pop().expect("No output!")
}

#[aoc(day9, part2)]
fn part2(mem: &[isize]) -> isize {

    let boost_mode = 2;
    let mut runner = OpcodeRunner::new(mem);

    let mut outputs: Vec<isize> = Vec::new();

    runner.push_input(boost_mode);

    while !runner.has_halted() {
        let cur_opcode = runner.parse_cur_opcode();
        let got_output = runner.exec_opcode(cur_opcode);
        if got_output {
            outputs.push(runner.output().unwrap());
        }
    }


//    dbg!(outputs);
    outputs.pop().expect("No output!")
}