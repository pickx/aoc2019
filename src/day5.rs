use crate::opcode::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(mem: &[isize]) -> Option<isize> {

    const INPUT: isize = 1;


    let mut runner = OpcodeRunner::new(mem);
    runner.set_input_consume_mode(InputMode::SingleInput);
    runner.push_input(INPUT);

    while !runner.has_halted() {
        let cur_opcode = runner.parse_cur_opcode();
        runner.run_opcode(cur_opcode);
    }

    runner.output()
}

#[aoc(day5, part2)]
fn part2(mem: &[isize]) -> Option<isize> {

    const INPUT: isize = 5;

    let mut runner = OpcodeRunner::new(mem);
    runner.set_input_consume_mode(InputMode::SingleInput);
    runner.push_input(INPUT);

    while !runner.has_halted() {
        let cur_opcode = runner.parse_cur_opcode();
        runner.run_opcode(cur_opcode);
    }

    runner.output()
}
