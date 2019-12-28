use std::iter;
use std::iter::{Chain, FromIterator};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn generate_signal(nums: &[i64], times_to_repeat: usize) -> Vec<i64> {
    let signal_length = times_to_repeat * nums.len();

    nums
        .iter()
        .cycle()
        .copied() //derefs elements automatically because they are Copy
        .take(signal_length)
        .collect()
}


fn fake_fft(signal: &[i64], offset: usize) -> Vec<i64> {
    let predictable_numbers = signal.len() - offset;
    assert!(predictable_numbers <= signal.len() / 2);
    let mut res = signal[offset..].to_vec();
    const PHASES: usize = 100;
    for _ in 0..PHASES {
        let mut next_sum: i64 = 0;
        for i in (0..res.len()).rev()  {
            next_sum += res[i];
            res[i] = next_sum % 10;
        }
    }


    res

}

fn find_offset(nums: &[i64]) -> usize {
    nums[0..7]
        .iter()
        .join("")
        .parse()
        .unwrap()
}

fn apply_pattern_with_offset(nums: &[i64]) -> String {

    let offset: usize = find_offset(&nums);

    let factor = 10_000;
    let signal_length = factor * nums.len();

    let signal = generate_signal(&nums, factor);

    let fft = fake_fft(&signal, offset);

    const DIGITS_TO_TAKE: usize = 8;

    fft.iter().take(DIGITS_TO_TAKE).join("")


}

fn apply_pattern_optimized(nums: &[i64]) -> String {
        const PHASES: usize = 100;
        const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];


        let mut cur_input = nums.to_vec();
        let digits = cur_input.len();

        for _ in 0..PHASES {

            let mut next_input = Vec::with_capacity(digits);

            //note the range limits
            for element_num in 1..digits+1 {

                let repeat_n_times = |num: &i64| iter::repeat(*num).take(element_num);

                let pattern = BASE_PATTERN
                    .iter()
                    .flat_map(repeat_n_times)
                    .cycle()
                    .skip(element_num - 1) //optimization
                    .skip(1);

                let sum: i64 = cur_input
                    .iter()
                    .skip(element_num - 1) //optimization
                    .zip(pattern)
                    .fold(0, |acc, (input_num, scalar)| acc + (input_num * scalar));

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
fn part1(nums: &[i64]) -> String {
    apply_pattern_optimized(&nums)
}

#[test]
fn part1_test1() {
    let input = "80871224585914546619083218645595";
    let nums = input_generator(input);
    let expected = "24176176".to_string();
    assert_eq!(part1(&nums), expected);
}

#[test]
fn part1_test2() {
    let input = "19617804207202209144916044189917";
    let nums = input_generator(input);
    let expected = "73745418".to_string();
    assert_eq!(part1(&nums), expected);
}

#[test]
fn part1_test3() {
    let input = "69317163492948606335995924319873";
    let nums = input_generator(input);
    let expected = "52432133".to_string();
    assert_eq!(part1(&nums), expected);
}

#[aoc(day16, part2)]
fn part2(nums: &[i64]) -> String {
    apply_pattern_with_offset(nums)
}

#[test]
fn part2_test1() {
    let input = "03036732577212944063491565474664";
    let nums = input_generator(input);
    let expected = "84462026".to_string();
    assert_eq!(part2(&nums), expected);
}

#[test]
fn part2_test2() {
    let input = "02935109699940807407585447034323";
    let nums = input_generator(input);
    let expected = "78725270".to_string();
    assert_eq!(part2(&nums), expected);
}

#[test]
fn part2_test3() {
    let input = "03081770884921959731165446850517";
    let nums = input_generator(input);
    let expected = "53553731".to_string();
    assert_eq!(part2(&nums), expected);
}
