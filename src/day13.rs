use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};
use crate::intcode::*;
use std::io;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl From<isize> for Tile {
    fn from(x: isize) -> Tile {
        match x {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unimplemented tile type."),
        }
    }
}

fn _draw(screen: &Vec<Vec<Tile>>) {
    for row in screen {
        for &tile in row {
            let graphic = match tile {
                Tile::Empty => " ",
                Tile::Wall => "|",
                Tile::Block => "X",
                Tile::Paddle => "+",
                Tile::Ball => "*",
            };

            print!("{}", graphic);
        }
        println!();
    }
}

fn create_screen() -> Vec<Vec<Tile>>{
    //these capacity values have been determined from the screen drawing in part1
    const NUM_ROWS: usize = 25;
    const NUM_COLS: usize = 40;

    let mut screen: Vec<Vec<Tile>> = Vec::with_capacity(NUM_ROWS);
    for _ in 0..NUM_ROWS {
        screen.push(vec![Tile::Empty; NUM_COLS]);
    }

    screen
}

//instead of playing the game, I realized that when the ball hits the floor,
//it sends Opcode 0. So rather than crash when receiving Opcode 0, I made my
//runner restart when Opcode 0 is received, however the memory remains,
//in other words, touching the floor no longer loses the game.
//so it plays itself.
fn play(runner: IntcodeRunner) -> isize {

    let mut runner = runner;
    let mut screen = create_screen();
    let mut score = 0;

    let mut buf = VecDeque::with_capacity(3);
    loop {
        while buf.len() < 3 {

            let next_opcode = runner.parse_cur_opcode();

            if let Opcode::In(_) = next_opcode {
//                let joystick_position = OpcodeRunner::ask_for_input();
                let joystick_position = 0; //hacked
                runner.push_input(joystick_position);
            }

            let got_output = runner.exec_opcode(next_opcode);

            if got_output {
                buf.push_back(runner.output().unwrap());
            }


        }

        if !runner.has_halted() {
            let (out1,
                out2,
                out3) =
                (buf.pop_front().unwrap(),
                 buf.pop_front().unwrap(),
                 buf.pop_front().unwrap());

            match (out1, out2) {

                (-1, 0) => {
                    score = out3;
                }

                _ => {
                    //note the order of variables!
                    let (row, col) = (out2 as usize, out1 as usize);
                    let t_type: Tile = out3.into();
                    screen[row][col] = t_type;
//                    _draw(&screen); //no longer needed since the game is not interactive anymore
                }

            }

        }

        else {
            break;
        }

    }


    score
}

#[aoc(day13, part1)]
pub fn part1(mem: &[isize]) -> usize {

    let mut tiles: HashMap<Pos, Tile> = HashMap::new();
    let mut buf = VecDeque::with_capacity(3);

    let mut runner = IntcodeRunner::new(mem);

    loop {
        while buf.len() < 3 {

            let next_opcode = runner.parse_cur_opcode();

            let got_output = runner.exec_opcode(next_opcode);

            if got_output {
                buf.push_back(runner.output().unwrap());
            }
        }

        if !runner.has_halted() {
            let (pos_x, pos_y) = (buf.pop_front().unwrap(), buf.pop_front().unwrap());
            let t_pos = Pos { x: pos_x as usize, y: pos_y as usize };
            let t_type: Tile = buf.pop_front().unwrap().into();
            tiles.insert(t_pos, t_type);
        }

        else {
            break;
        }

    }


    tiles
        .values()
        .filter(|&&tile| tile == Tile::Block )
        .count()
}

#[aoc(day13, part2)]
pub fn part2(mem: &[isize]) -> isize {

    let mut mem = mem.to_vec();

    //"Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free."
    mem[0] = 2;

    let runner = IntcodeRunner::new(&mem);

    play(runner) //games plays itself, ends up returning the final score


}

