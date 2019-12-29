use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::IntcodeRunner;
use itertools::all;

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

fn _draw(image: Vec<Vec<char>>) {
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

//    _draw(image);

    let intersections = find_scaffold_intersections(image);

    intersections
        .iter()
        .fold(0, |acc, &Pos { row, col }| acc + (row * col))
}
