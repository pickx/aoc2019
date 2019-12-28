use aoc_runner_derive::{aoc, aoc_generator};

use crate::opcode::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use itertools::Itertools;


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

enum MovementResult {
    HitWall,
    MoveSuccess,
    FoundOxygenSystem,
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

fn positions_from(pos: Pos, maze: &HashMap<Pos, Tile>) -> Vec<_>{
    let (x, y) = (pos.x, pos.y);
    let positions = [(x, y-1), (x, y+1), (x-1, y), (x+1, y)];
    let movement_codes = [1_isize, 2, 3, 4];

     positions
        .iter()
        .zip(movement_codes.iter())
        .filter(|&(pos, _)| !maze.contains_key(pos))
        .collect_vec()
}

fn dfs(maze: &mut HashMap<Pos, Tile>, runner: from: Pos, distance: usize) {




}




#[aoc(day15, part1)]
fn part1(mem: &[isize]) -> usize {

//    let mut maze: HashMap<Pos, Tile> = HashMap::new();
//    let mut runner = OpcodeRunner::new(mem);
//    runner.set_input_consume_mode(InputMode::SingleInput);
//    let starting_position = (0, 0);
//    let mut robot_pos: Pos = starting_position;
//
//
//
//    let mut lookup_table: HashMap<char, isize> = HashMap::new();
//    lookup_table.insert('n', 1);
//    lookup_table.insert('N', 1);
//    lookup_table.insert('s', 2);
//    lookup_table.insert('S', 2);
//    lookup_table.insert('w', 3);
//    lookup_table.insert('W', 3);
//    lookup_table.insert('e', 4);
//    lookup_table.insert('E', 4);
//
//    loop {
//        maze.insert(robot_pos, Tile::Robot);
//
//        let image = image(&maze, robot_pos);
//        draw(image);
//
//        let (x, y) = (robot_pos.0, robot_pos.1);
//        let positions = [(x, y-1), (x, y+1), (x-1, y), (x+1, y)];
//        let movement_codes = [1_isize, 2, 3, 4];
//        let mut unexplored_positions = positions
//            .iter()
//            .zip(movement_codes.iter())
//            .filter(|&(pos, _)| !maze.contains_key(pos))
//            .collect_vec();
//
//
//        for (_, movement_code) in unexplored_positions {
//
//            let movement: Pos = match movement_code {
//                1 => (0, -1),
//                2 => (0, 1),
//                3 => (-1, 0),
//                4 => (1, 0),
//                _ => panic!("Illegal movement"),
//            };
//
//            runner.push_input(*movement_code);
//            let got_output = false;
//
//            let movement_result: MovementResult = loop {
//                let next_opcode = runner.parse_cur_opcode();
//
//                let got_output = runner.exec_opcode(next_opcode);
//
//                if got_output {
//                    break match runner.output().unwrap() {
//                        0 => MovementResult::HitWall,
//                        1 => MovementResult::MoveSuccess,
//                        2 => MovementResult::FoundOxygenSystem,
//                        x => panic!("Got unsupported output {}", x),
//                    };
//                }
//            };
//
//            let new_pos = (robot_pos.0 + movement.0, robot_pos.1 + movement.1);
//
//            match movement_result {
//                MovementResult::HitWall => {
//                    maze.insert(new_pos, Tile::Wall);
//                },
//
//                MovementResult::MoveSuccess => {
//                    maze.insert(robot_pos, Tile::Empty);
//                    robot_pos = new_pos;
//                }
//
//                MovementResult::FoundOxygenSystem => {
//                    maze.insert(new_pos, Tile::OxygenSystem);
//                    let oxygen_system_pos = new_pos;
////                return manhattan_distance(starting_position, new_pos);
//                    println!("Found OxygenSystem at ({}, {})", new_pos.0, new_pos.1);
////                    return dfs(starting_position, oxygen_system_pos);
//                    return 0;
//                }
//            }
//
//
//        }
//
//
////        let movement_code = loop {
////            if let Ok(code) = OpcodeRunner::ask_for_input(&lookup_table) {
////                break code
////            }
////        };






//}

}