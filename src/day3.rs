use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

const GRID_HEIGHT: usize = 30000;
const GRID_LENGTH: usize = 30000;

#[allow(non_snake_case)]
#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Steps {
    direction: Dir,
    count: usize,
}

impl Dir {
    fn step_vert(&self) -> i32 {
        match self {
            Dir::U => -1,
            Dir::D => 1,
            _ => 0,
        }
    }

    fn step_hori(&self) -> i32 {
        match self {
            Dir::L => -1,
            Dir::R => 1,
            _ => 0,
        }
    }
}

impl Into<Steps> for &str {
    fn into(self) -> Steps {

        let (dir_str, step_count_str) = self.split_at(1);


        let step_count: usize = step_count_str.parse().expect("Parse error at step count.");

        let dir = match dir_str {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!("Parse error at direction.")
        };

        Steps { direction: dir, count: step_count }

    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> (Vec<Steps>, Vec<Steps>) {

    let (first_path_str, second_path_str): (&str, &str) = input.lines().collect_tuple().unwrap();

    let first_path = first_path_str.split(',').map(|str| str.into()).collect();
    let second_path = second_path_str.split(',').map(|str| str.into()).collect();

    (first_path, second_path)
}

#[aoc(day3, part1)]
fn part1(path_pair: &(Vec<Steps>, Vec<Steps>)) -> usize {
    let (first_path, second_path) = path_pair;

    let mut grid = vec![vec![false; GRID_LENGTH]; GRID_HEIGHT];

    let (origin_row, origin_col) = (GRID_HEIGHT/2, GRID_LENGTH/2);

    let (mut row, mut col) = (origin_row, origin_col);

    grid[row][col] = true; //origin point is an intersection

    for s in first_path {
        for _ in 1..=s.count {
            row = (s.direction.step_vert() + row as i32) as usize;
            col = (s.direction.step_hori() + col as i32) as usize;
            grid[row][col] = true;
        }
    }

    let mut intersections: Vec<(usize, usize)> = vec![];

    row = origin_row;
    col = origin_col;

    for s in second_path {
        for _ in 1..=s.count {
            row = (s.direction.step_vert() + row as i32) as usize;
            col = (s.direction.step_hori() + col as i32) as usize;
            if grid[row][col] {
                intersections.push((row, col));
            }
        }
    }

    let mut min_dist = usize::max_value();
    for (row, col) in intersections {
        let cur_dist = (row.max(origin_row) - row.min(origin_row))
                             + (col.max(origin_col) - col.min(origin_col));

        if cur_dist > 0 {
            min_dist = min_dist.min(cur_dist);
        }

    }

        min_dist

}


#[aoc(day3, part2)]
fn part2(path_pair: &(Vec<Steps>, Vec<Steps>)) -> usize {

    let (first_path, second_path) = path_pair;

    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_LENGTH]; GRID_HEIGHT];

    let (origin_row, origin_col) = (GRID_HEIGHT/2, GRID_LENGTH/2);

    let (mut row, mut col) = (origin_row, origin_col);
    let mut step_meter = 0;

    for s in first_path {
        for _ in 1..=s.count {
            step_meter += 1;
            row = (s.direction.step_vert() + row as i32) as usize;
            col = (s.direction.step_hori() + col as i32) as usize;
            grid[row][col] = step_meter;
        }
    }

    let mut intersections: Vec<usize> = vec![];

    row = origin_row;
    col = origin_col;
    step_meter = 0;
    for s in second_path {
        for _ in 1..=s.count {
            step_meter += 1;
            row = (s.direction.step_vert() + row as i32) as usize;
            col = (s.direction.step_hori() + col as i32) as usize;
            if grid[row][col] != 0 {
                intersections.push(grid[row][col] + step_meter);
            }
        }
    }

    *(intersections.iter().min().unwrap())
}

