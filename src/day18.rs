use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Entrance,
    Passage,
    Wall,
    Key(char),
    Door(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Tile::Entrance
            '.' => Tile::Passage,
            '#' => Tile::Wall,
            alpha if alpha.is_lowercase() => Tile::Key(alpha),
            alpha if alpha.is_uppercase() => Tile::Door(alpha),
            _ => panic!("Unimplemented"),
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Entrance => '@',
            Tile::Passage => '.',
            Tile::Wall => '#',
            Tile::Key(alpha) || Tile::Door(alpha) => alpha,
        }
    }
}