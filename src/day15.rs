use aoc_runner_derive::{aoc, aoc_generator};

use crate::opcode::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use itertools::Itertools;
use crate::day15::MoveResult::{MoveSuccess, HitWall};

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Robot,
    Wall,
    Empty,
    OxygenSystem,
}

impl TryFrom<&Tile> for char {
    type Error = &'static str;

    fn try_from(t: &Tile) -> Result<char, Self::Error> {

        match t {
            Tile::Robot => Ok('D'),
            Tile::Wall => Ok('#'),
            Tile::Empty => Ok('.'),
            Tile::OxygenSystem => Ok('O'),
        }
    }
}

type Pos = (isize, isize); // (x,y ) position relative to starting point (0, 0)

fn image(maze: &HashMap<Pos, Tile>, robot_pos: Pos) -> Vec<Vec<char>> {
    let unexplored: char = ' ';
    let dist: isize = 3;
    let image_dim = 2 * (dist as usize) + 1;
    let mut image = vec![vec![unexplored; image_dim]; image_dim];

    let (rob_x, rob_y) = (robot_pos.0, robot_pos.1);

    for (row, y) in ((rob_y - dist)..=(rob_y + dist)).enumerate() {
            for (col, x) in ((rob_x - dist)..=(rob_x + dist)).enumerate() {
                if let Some(tile) = maze.get(&(x, y)) {
                    image[row][col] = char::try_from(tile).unwrap();
                }
            }
        }
    image
}

fn draw(image: Vec<Vec<char>>) {
    for row in image {
        for c in row {
            print!("{}", c);
        }

        println!();
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MoveResult {
    HitWall,
    MoveSuccess,
    FoundOxygenSystem,
}

impl MoveResult {
    fn to_corresponding_tile(&self) -> Tile {
        match self {
            MoveResult::HitWall => Tile::Wall,
            MoveResult::MoveSuccess => Tile::Empty,
            MoveResult::FoundOxygenSystem => Tile::OxygenSystem,
        }
    }

}

impl From<isize> for MoveResult {
    fn from(response_code: isize) -> Self {
        match response_code {
            0 => MoveResult::HitWall,
            1 => MoveResult::MoveSuccess,
            2 => MoveResult::FoundOxygenSystem,
            x => panic!("Got unsupported output {}", x),
        }
    }
}

impl From<MoveResult> for isize {
    fn from(move_result: MoveResult) -> Self {
        match move_result {
            MoveResult::HitWall => 0 ,
            MoveResult::MoveSuccess => 1,
            MoveResult::FoundOxygenSystem => 2,
        }
    }
}

fn movement_code_to_direction(movement_code: isize) -> Pos {
    match movement_code {
        1 => (0, -1),
        2 => (0, 1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => panic!("Illegal movement"),
    }
}


fn positions_from(pos: Pos) -> Vec<(Pos, isize)>{
    let (x, y) = (pos.0, pos.1);

    let positions = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
    let movement_codes = [1_isize, 2, 3, 4];

    positions.iter().cloned()
        .zip(movement_codes.iter().cloned())
        .collect()
}

fn advance_runner_until_output(runner: &mut OpcodeRunner, input: isize) -> MoveResult {
    runner.push_input(input);
    let got_output = false;

    loop {
        let next_opcode = runner.parse_cur_opcode();

        let got_output = runner.exec_opcode(next_opcode);

        if got_output {
            let output = runner.output().unwrap();
            return MoveResult::from(output);
        }
    }
}

fn dfs(maze: &mut HashMap<Pos, Tile>,
       distances: &mut HashMap<Pos, usize>,
       cur_pos: Pos,
       mut runner: OpcodeRunner,
       code_to_send: isize,
       distance: usize) {

    //first, check if we've already been here.
    if let Some(&tile) = maze.get(&cur_pos) {
        if tile == Tile::Wall {
            //we've been here and it's a wall, so we can't advance any further.
            // no need to bother with this position
            return;
        }

        //we've already been here so it's safe to unwrap()
        let best_known_distance_to_here = distances.get(&cur_pos).unwrap();
        if &distance >= best_known_distance_to_here {
            //we've been here but haven't been able to do any better, distance-wise.
            //this also means that we've mapped all positions from this position
            return;
        }
    }

    //now try stepping to this location.
    let response = advance_runner_until_output(&mut runner, code_to_send);
    if response == MoveResult::HitWall {
        maze.insert(cur_pos, Tile::Wall);
        //then we can't advance any further
        return;
    }

    let tile_here = response.to_corresponding_tile();
    maze.entry(cur_pos).or_insert(tile_here);
    distances.insert(cur_pos, distance);

    if response == MoveResult::HitWall {
        //then we can't advance any further
        return;
    }


    let adjacent_positions = positions_from(cur_pos);
    for (position, code) in adjacent_positions {
        dfs(maze, distances, position, runner.clone(),code, distance + 1);
    }

}




#[aoc(day15, part1)]
fn part1(mem: &[isize]) -> usize {

    let starting_position = (0, 0);

    let mut maze: HashMap<Pos, Tile> = HashMap::new();
    maze.insert(starting_position, Tile::Empty);

    let mut distances: HashMap<Pos, usize> = HashMap::new();
    distances.insert(starting_position, 0);

    let mut runner = OpcodeRunner::new(mem);
    runner.set_input_consume_mode(InputMode::SingleInput);


    let adjacent_positions = positions_from(starting_position);
    for (position, code) in adjacent_positions {
        dfs(&mut maze, &mut distances, position, runner.clone(),code, 1);
    }

    for (pos, dist) in distances {
        if maze[&pos] == Tile::OxygenSystem {
            return dist;
        }
    }

    usize::max_value()
}