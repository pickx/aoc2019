#[macro_use]
extern crate scan_fmt;
extern crate aoc_runner_derive;
extern crate itertools;

#[doc(inline)]
pub use std;

use aoc_runner_derive::aoc_lib;


mod day5;
mod day6;
//mod day7;
mod day8;
mod day10;
mod day12;


aoc_lib! { year = 2019 }