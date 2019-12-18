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

pub fn part1(input: &str) -> u32 {
    let total_fuel: u32 = input.
        lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .map(|mass| mass_to_fuel(mass))
        .sum();

    total_fuel
}

pub fn part2(input: &str) -> u32 {
    let total_fuel_recursive: u32 = input.
        lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .map(|mass| fuel_calc_recursive(mass))
        .sum();

    total_fuel_recursive
}