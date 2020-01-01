use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {

    input.lines().collect()
}


//#[aoc_generator(day18)]
//pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
//    let line_parser = |line: &str| {
//        line
//            .chars()
//            .map(|c| c.into())
//            .collect::<Vec<Tile>>()
//    };
//
//    input.lines().map(line_parser).collect()
//}

//#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
//pub enum Tile {
//    Entrance,
//    Passage,
//    Wall,
//    Key(char),
//    Door(char),
//}
//
//impl From<char> for Tile {
//    fn from(c: char) -> Self {
//        match c {
//            '@' => Tile::Entrance,
//            '.' => Tile::Passage,
//            '#' => Tile::Wall,
//            alpha if alpha.is_lowercase() => Tile::Key(alpha),
//            alpha if alpha.is_uppercase() => Tile::Door(alpha),
//            _ => panic!("Unimplemented"),
//        }
//    }
//}
//
//impl From<Tile> for char {
//    fn from(t: Tile) -> Self {
//        match t {
//            Tile::Entrance => '@',
//            Tile::Passage => '.',
//            Tile::Wall => '#',
//            Tile::Key(alpha) | Tile::Door(alpha) => alpha,
//        }
//    }
//}

//fn bfs(distance: usize) ->

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

fn get<T>(v: &Vec<Vec<T>>, at_pos: Pos) -> &T {
    &v[at_pos.row][at_pos.col]
}

fn find_entrance(maze: &Vec<Vec<char>>) -> Pos {
    let mut pos = Pos { row: 0, col: 0 };
    for row in 0..maze.len() {
        for col in 0..maze[0].len() {
            if maze[row][col] == '@' {
                pos = Pos { row, col };
            }
        }
    }
    pos
}

#[aoc(day18, part1)]
pub fn part1(maze: &Vec<Vec<char>>) -> usize {
    let mut maze = maze.clone();
    let mut distances: HashMap<Pos, usize> = HashMap::new();
    let mut reachable_objects: HashMap<char, Pos> = find_reachable_objects(stuff); //keys and doors

    let mut cur_pos = find_entrance(&maze);

    let doors = |hm: &HashMap<char, _>| hm.keys().filter(|&&c| c.is_uppercase());
    let keys = |hm: HashMap<char, _>| hm.keys().filter(|&&c| c.is_lowercase());

    let mut distances_walked: Vec<usize> = Vec::new();

    while !reachable_objects.is_empty() {
        let mut keys_to_consider: Vec<char> = Vec::new();
        for &door in doors(&reachable_objects) {
            let ref matching_key = door.to_ascii_lowercase();
            if reachable_objects.contains_key(matching_key) {
                keys_to_consider.push(*matching_key);
            }
        }

        if keys_to_consider.is_empty() {
            break;
        }

        for key in keys_to_consider {
            let key_pos = reachable_objects[&key];
            let matching_door_pos = reachable_objects[&key.to_ascii_uppercase()];

            

        }

    }

    distances_walked.iter().sum()
}
