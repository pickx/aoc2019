use aoc_runner_derive::{aoc, aoc_generator};
use crate::opcode::{OpcodeRunner};
use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn step(&mut self, dir: Direction) {

        let next_x = self.col as isize + dir.x_dir();
        let next_y = self.row as isize + dir.y_dir();

        assert!(next_x >= 0);
        assert!(next_y >= 0);

        self.col = next_x as usize;
        self.row = next_y as usize;

    }
}

impl Direction {

    fn y_dir(&self) -> isize {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    fn x_dir(&self) -> isize {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }


    fn turn_left(&mut self) {
        *self = match self {
           Direction::Up => Direction::Left,
           Direction::Left => Direction::Down,
           Direction::Down => Direction::Right,
           Direction::Right => Direction::Up,
       };
    }

    fn turn_right(&mut self) {
        *self = match self {
           Direction::Up => Direction::Right,
           Direction::Right => Direction::Down,
           Direction::Down => Direction::Left,
           Direction::Left => Direction::Up,
       };
    }
}

struct Robot {
    facing: Direction,
    color_to_paint: Color,
    pos: Pos,
    runner: OpcodeRunner,
}

impl Robot {
    pub fn new(start_pos: Pos, mem: &[isize]) -> Robot {
        let default_facing = Direction::Up;

        let runner = OpcodeRunner::new(mem);

        Robot { facing: default_facing, color_to_paint: Color::Black, pos: start_pos, runner }
    }

    fn emit(&mut self) -> (isize, isize) {

        let mut outputs = Vec::with_capacity(2);

        while outputs.len() < 2 {
            let next_opcode = self.runner.parse_cur_opcode();

            let got_output = self.runner.exec_opcode(next_opcode);

            if got_output {
                outputs.push(self.runner.output().unwrap());
            }
        }

        (outputs[0], outputs[1])

    }

    fn update_instructions(&mut self) {
        let (paint_bit, turn_bit) = self.emit();

        match paint_bit {
            0 => self.color_to_paint = Color::Black,
            1 => self.color_to_paint = Color::White,
            _ => panic!("Got illegal paint bit"),
        };

        match turn_bit {
            0 => self.facing.turn_left(),
            1 => self.facing.turn_right(),
            _ => panic!("Got illegal turn bit"),
        };
    }

    pub fn has_halted(&self) -> bool {
        self.runner.has_halted()
    }

    pub fn paint_and_step_forward(&mut self, grid: &mut Vec<Vec<Color>>) -> Pos {
        let grid_rows = grid[0].len();
        let grid_cols = grid.len();

        if self.pos.col >= grid_cols || self.pos.row >= grid_rows {
            panic!("Robot outside grid");
        }

        let pos_painted = self.pos;

        let input_to_use = match &grid[self.pos.row][self.pos.col] {
            Color::Black => 0,
            Color::White => 1,
        };
        self.runner.push_input(input_to_use);
        self.update_instructions();

        grid[self.pos.row][self.pos.col] = self.color_to_paint;

        self.step_forward();

        pos_painted
    }

    fn step_forward(&mut self) {
        let current_direction = self.facing;
        self.pos.step(current_direction);
    }

}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Color {
    Black,
    White,
}

fn _painter(grid: &Vec<Vec<Color>>) {
    for row in grid {
        for p in row {
            match p {
                Color::Black => {print!(".");},
                Color::White => {print!("#");},
            }

        }
        println!();
    }
}

#[aoc(day11, part1)]
fn part1(mem: &[isize]) -> usize {
    let (grid_cols, grid_rows) = (150, 150);
    let mut grid = vec![vec![Color::Black; grid_cols]; grid_rows];
    let starting_pos = Pos { row: grid_rows / 2, col: grid_cols / 2 };

    let mut robot = Robot::new(starting_pos, mem);

    let mut painted: HashSet<Pos> = HashSet::new();

    while !robot.has_halted() {
        let pos_painted = robot.paint_and_step_forward(&mut grid);
        painted.insert(pos_painted);
    }

    painted.len()
}

#[aoc(day11, part2)]
fn part2(mem: &[isize]) -> &'static str {
    let (grid_cols, grid_rows) = (50, 50);
    let mut grid = vec![vec![Color::Black; grid_cols]; grid_rows];
    let starting_pos = Pos { row: 0, col: 0 };

    grid[starting_pos.row][starting_pos.col] = Color::White; //robot starts on a single white space in part 2

    let mut robot = Robot::new(starting_pos, mem);

    while !robot.has_halted() {
        robot.paint_and_step_forward(&mut grid);
    }

    //_painter(&grid); //paints "FKEKCFRK"
    "FKEKCFRK"
}