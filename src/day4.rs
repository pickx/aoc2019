use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;
use itertools::Itertools;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Range<usize> {
    let (range_start, range_end): (usize, usize) = input
        .split('-')
        .map(|num| num.parse())
        .map(|res| res.unwrap())
        .collect_tuple()
        .unwrap();

    range_start..range_end

}

fn contains_adjacent_digits(v: &[usize]) -> bool {
    v
        .windows(2)
        .any(|w| w[0] == w[1])
}

fn is_nondecreasing(v: &[usize]) -> bool {
    v
        .windows(2)
        .all(|w| w[0] <= w[1])
}

fn contains_group_of_exactly_2_digits(v: &[usize]) -> bool {
    let mut i = 1;

    while i < v.len() {


        if v[i] == v[i-1] {

            //if we found 2 equal, adjacent elements, then check if it's not part of a
            //group of more than 2 equal elements.
            if i == v.len() - 1 || v[i+1] != v[i] {
                return true;
            }

            //otherwise, advance to the next pair that satisfies this property
            while i < v.len() && v[i] == v[i-1] {
                i += 1;
            }
        }

        else {
            i += 1;
        }

    }

    false
}

fn to_vec(x: usize) -> Vec<usize> {
    format!("{}", x)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d as usize)
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(range: &Range<usize>) -> usize {

    let mut valid_passes = vec![];
    for pass in range.start..range.end {

        let pass_vec = to_vec(pass);

        if contains_adjacent_digits(&pass_vec) && is_nondecreasing(&pass_vec) {
            valid_passes.push(pass_vec);
        }

    }

    valid_passes.len()
}

#[aoc(day4, part2)]
pub fn part2(range: &Range<usize>) -> usize {
    let mut valid_passes = vec![];
    for pass in range.start..range.end {

        let pass_vec = to_vec(pass);

        if contains_adjacent_digits(&pass_vec) && is_nondecreasing(&pass_vec) {
            valid_passes.push(pass_vec);
        }

    }

    let mut valid_passes_by_new_rule = vec![];

    for pass in valid_passes {
        if contains_group_of_exactly_2_digits(&pass) {
            valid_passes_by_new_rule.push(pass.clone());
        }
    }

    valid_passes_by_new_rule.len()
}