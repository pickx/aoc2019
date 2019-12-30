use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::IntcodeRunner;
use itertools::{all, Itertools};
use std::io;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

fn get_camera_image(mem: &[isize]) -> Vec<Vec<char>> {
    let mut image: Vec<Vec<char>> = Vec::new();
    let mut runner = IntcodeRunner::new(mem);

    let mut buf: Vec<char> = Vec::new();
    while !runner.has_halted() {

        let next_pixel =
        loop {
            let next_opcode = runner.parse_cur_opcode();

            let got_output = runner.exec_opcode(next_opcode);

            if got_output {
                let output_u8 = runner.output().unwrap() as u8;
                break output_u8
            }
        };

        match next_pixel {
            10 => {
                image.push(buf);

                //just in case it wasn't obvious, this just change what "buf" is pointing at
                //in other words, the previous buf lives on, now owned by "image"
                buf = Vec::new();
            }

            _ => {
                buf.push(next_pixel as char);
            }
        };

    }

    //the way the input is sent, an empty row would be added as the last element
    image.pop();

    image
}

fn _draw(image: &Vec<Vec<char>>) {
    for row in image {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {

    fn adjacent(&self) -> Vec<Pos> {

        let mut adjacent = Vec::new();
        let (r, c) = (self.row as isize, self.col as isize);
        for &(row, col) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].iter() {
            if row >= 0 && col >= 0 {
                let adjacent_pos = Pos { row: row as usize, col: col as usize };
                adjacent.push(adjacent_pos);
            }
        }

        adjacent
    }

}

fn find_scaffold_intersections(image: Vec<Vec<char>>) -> Vec<Pos> {
    let mut intersections = Vec::new();
    let (rows, cols) = (image.len(), image[0].len());
    //intersections can't happen on edges of image... I think.
    //so this simplifies checking
    for row in 1..rows-1 {
        for col in 1..cols-1 {

            let cur_pos = Pos { row, col };

            let is_hash_at_pos = |Pos { row, col }| image[row][col] == '#';

            if is_hash_at_pos(cur_pos) && all(cur_pos.adjacent().iter().copied(), is_hash_at_pos) {
                intersections.push(cur_pos);
            }
        }
    }



    intersections
}

#[aoc(day17, part1)]
fn part1(mem: &[isize]) -> usize {
    let image = get_camera_image(mem);

    _draw(&image);

    let intersections = find_scaffold_intersections(image);

    intersections
        .iter()
        .fold(0, |acc, &Pos { row, col }| acc + (row * col))
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    fn draw(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

fn _interactive_step(mut image: Vec<Vec<char>>) {
    let pos = {
        let mut pos = None;

        'outer: for row in 0..image.len() {
            for col in 0..image[0].len() {
                if image[row][col] == '^' {
                    pos = Some(Pos { row, col });
                    break 'outer;
                }
            }
        }
        pos
    };

    let mut pos = pos.expect("Robot position not found");

    let mut dir = Direction::Up;
    let mut instructions = Vec::new();

    loop {
        image[pos.row][pos.col] = dir.draw();
        _draw(&image);
        printer(&instructions);

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_e) => panic!(_e.to_string()),
        }

        input = input.trim_end().to_string();

        let mut chars = input.chars();
        let first_char = chars.next();
        let steps = chars.as_str().parse::<usize>();
        if first_char.is_none() || steps.is_err() {
            println!("Wrong input: steps or first_char. input was {}", input);
            continue;
        }

        match first_char.unwrap() {
            'L' => dir.turn_left(),
            'R' => dir.turn_right(),
            'S' => break,
            _ => {
                println!("Wrong input: first_char");
                continue;
            }
        };

        instructions.push(input);

        pos = {
            let (mut r, mut c) = (pos.row as isize, pos.col as isize);
            for _ in 0..steps.unwrap() {
                image[r as usize][c as usize] = 'X';
                r += dir.y_dir();
                c += dir.x_dir();
            }

            Pos { row: r as usize, col: c as usize}
        };

    }

}

fn printer(v: &Vec<String>) {
    println!("{}", v.join(","));
}

#[aoc(day17, part2)]
fn part2(mem: &[isize]) -> usize {

    //part 2 requires changing memory address 0 from 1 to 2.
    let mut mem = mem.to_vec();
    mem[0] = 2;
    let mut runner = IntcodeRunner::new(&mem);

    //the following solutions were worked out by hand.
    let main_movement = ""


    0
}