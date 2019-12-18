#[macro_use]
extern crate scan_fmt;
extern crate aoc_runner_derive;
extern crate itertools;

use aoc_runner_derive::aoc_lib;

mod opcode;

mod day1;
//mod day 2;
mod day4;
mod day3;
//mod day4;
mod day5;
mod day6;
//mod day7;
mod day8;
mod day10;
mod day12;

aoc_lib! { year = 2019 }
