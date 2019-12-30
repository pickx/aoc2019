use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{IntcodeRunner, Opcode};
use std::io;
use std::io::Error;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

//if let Opcode::In(_) = next_opcode {
//            let mut user_input = String::new();
//            match io::stdin().read_line(&mut user_input) {
//                Ok(_) => (),
//                Err(e) => panic!(e.to_string()),
//            }
//
//            let mut input = user_input.trim_end().parse::<isize>();
//            while input.is_err() {
//                match io::stdin().read_line(&mut user_input) {
//                    Ok(_) => (),
//                    Err(e) => panic!(e.to_string()),
//                }
//                input = user_input.trim_end().parse::<isize>();
//            }
//
//            runner.push_input(input.unwrap());
//        }

fn affected_points_in_area(area_rows: usize, area_cols: usize, mem: &[isize]) -> Vec<Vec<bool>> {
    let mut area = vec![vec![false; area_cols]; area_rows];

    for row in 0..area_rows {
        for col in 0..area_cols {
            let mut runner = IntcodeRunner::new(mem);
            runner.push_input(col as isize);
            runner.push_input(row as isize);
            while !runner.has_halted() {
                let next_opcode = runner.parse_cur_opcode();

                let got_output = runner.exec_opcode(next_opcode);

                if got_output {
                    let output = runner.output().unwrap();
                    area[row][col] = output == 1;
                }
            }
        }
    }

    area
}

#[aoc(day19, part1)]
fn part1(mem: &[isize]) -> usize {

    let (area_rows, area_cols) = (50, 50);
    let area = affected_points_in_area(area_rows, area_cols, mem);

    area
        .iter()
        .flatten()
        .filter(|&&b| b)
        .count()
}

#[aoc(day19, part2)]
fn part2(mem: &[isize]) -> usize {


}