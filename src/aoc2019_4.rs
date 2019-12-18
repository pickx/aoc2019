use crate::inputs::INPUT_2019_4;

fn parse_input(input: &str) -> (usize, usize) {

    let ranges: Vec<usize> = input.split('-').map(|num| num.parse().unwrap()).collect();
    assert_eq!(ranges.len(), 2);
    (ranges[0], ranges[1])

}

fn contains_adjacent_digits(v: &Vec<usize>) -> bool {
    v
        .windows(2)
        .any(|w| w[0] == w[1])
}

fn is_nondecreasing(v: &Vec<usize>) -> bool {
    v
        .windows(2)
        .all(|w| w[0] <= w[1])
}

fn contains_group_of_exactly_2_digits(v: &Vec<usize>) -> bool {
    let mut i = 1;

    while i < v.len() {

        if v[i] == v[i-1] {

            if i == v.len() - 1 || v[i+1] != v[i] {
                return true;
            }

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

pub fn part1() -> usize {
    let (range_start, range_end) = parse_input(INPUT_2019_4);

    let mut valid_passes = vec![];
    for pass in range_start..=range_end {

        let pass_vec = to_vec(pass);

        if contains_adjacent_digits(&pass_vec) && is_nondecreasing(&pass_vec) {
            valid_passes.push(pass_vec);
        }

    }

    valid_passes.len()
}

pub fn part2() -> usize {
    let (range_start, range_end) = parse_input(INPUT_2019_4);

    let mut valid_passes = vec![];
    for pass in range_start..=range_end {

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