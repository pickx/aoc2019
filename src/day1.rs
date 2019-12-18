use aoc_runner_derive::{aoc, aoc_generator};

fn mass_to_fuel(mass: u32) -> u32 {
    (mass / 3)
        .checked_sub(2)
        .unwrap_or(0)
}

fn fuel_calc_recursive(fuel: u32) -> u32 {
    let mut cur_fuel = fuel;
    let mut total_fuel = 0;
    while cur_fuel > 0 {
        let next_fuel = mass_to_fuel(cur_fuel);
        total_fuel += next_fuel;
        cur_fuel = next_fuel;
    }

    total_fuel
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mass = input
        .lines()
        .map(|line| line.parse::<u32>())
        .map(|parse_res| parse_res.unwrap())
        .collect();

    mass
}

#[aoc(day1, part1)]
pub fn part1(mass: &[u32]) -> u32 {
    mass
        .iter()
        .map(|&mass_unit| mass_to_fuel(mass_unit))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(mass: &[u32]) -> u32 {
        mass
        .iter()
        .map(|&mass_unit| fuel_calc_recursive(mass_unit))
        .sum()
}