use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter;
use std::iter::Chain;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

//fn generate_pattern(base_pattern: &[i32], elem_number: usize) -> [i32]{
//    let pattern: Vec<i32> = Vec::new();
//    for d in base_pattern {
//        for _ in 0..elem_number {
//            pattern.push(d);
//        }
//    }
//    pattern[1+elem_number..]
//}

fn apply_pattern(nums: &[i32]) -> String {
    const PHASES: usize = 100;
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];


    let mut cur_input = nums.to_vec();
    let digits = cur_input.len();

    for _ in 0..PHASES {
        let mut next_input = Vec::with_capacity(digits);

        //note the range limits
        for element_num in 1..digits+1 {
//            this was fun
            let pattern = BASE_PATTERN
                .iter()
                .flat_map(|num| iter::repeat(num).take(element_num))
                .cycle()
                .skip(1);

            let sum: i32 = cur_input
                .iter()
                .zip(pattern)
                .map(|(input_num, scalar)| input_num * scalar)
                .sum();

            let first_digit = (sum % 10).abs();

            next_input.push(first_digit);
        }

        cur_input = next_input;
    }


    const DIGITS_TO_TAKE: usize = 8;
    cur_input
        .iter()
        .take(DIGITS_TO_TAKE)
        .join("")
}

#[aoc(day16, part1)]
fn part1(nums: &[i32]) -> String {

    apply_pattern(&nums)
}

#[test]
fn part1_test1() {
    let input = "80871224585914546619083218645595";
    let nums = input_generator(input);
    let expected = "24176176".to_string();
    assert_eq!(apply_pattern(&nums), expected);
}

#[test]
fn part1_test2() {
    let input = "19617804207202209144916044189917";
    let nums = input_generator(input);
    let expected = "73745418".to_string();
    assert_eq!(apply_pattern(&nums), expected);
}

#[test]
fn part1_test3() {
    let input = "69317163492948606335995924319873";
    let nums = input_generator(input);
    let expected = "52432133".to_string();
    assert_eq!(apply_pattern(&nums), expected);
}