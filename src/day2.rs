use crate::opcode::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

pub fn run_with_noun_and_verb(mem: &[isize], noun: isize, verb: isize) -> isize {
    let mut runner = OpcodeRunner::new(mem);

    runner.set_noun(noun);
    runner.set_verb(verb);

    while !runner.has_halted() {
        let cur_opcode = runner.parse_cur_opcode();
        runner.exec_opcode(cur_opcode);
    }

    runner.value_at_pos_0()
}

#[aoc(day2, part1)]
pub fn part1(mem: &[isize]) -> isize {
    run_with_noun_and_verb(mem, 12, 2)
}

#[aoc(day2, part2)]
pub fn part2(mem: &[isize]) -> isize {
    const PUZZLE_OUTPUT: isize = 19_690_720;

    for verb in 0..=99 {
        for noun in 0..=99 {
            if run_with_noun_and_verb(mem, noun, verb) == PUZZLE_OUTPUT {
                return (noun * 100) + verb;
            }
        }
    }

    panic!("Not found");
}
