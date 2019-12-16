use aoc_runner_derive::{aoc, aoc_generator};
extern crate num;
use num::Integer;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum MapLoc {
    Empty,
    Asteroid,
}

impl MapLoc {
    fn from(c: char) -> MapLoc {
        match c {
            '.' => MapLoc::Empty,
            '#' => MapLoc::Asteroid,
            _ => panic!("Unrecognized character"),
        }
    }
}

type Slope = (usize, usize);

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn slope_to(&self, t: &Pos) -> Slope {
        let x_diff = self.x.dist_from(t.x);
        let y_diff = self.y.dist_from(t.y);

        match (x_diff, y_diff) {
            (0, 0) => (0, 0),
            (0, _) => (0, 1),
            (_, 0) => (1, 0),
            (_, _) => {
                let gcd = if x_diff >= y_diff {
                    x_diff.gcd(&y_diff)
                } else {
                    y_diff.gcd(&x_diff)
                };
                (x_diff / gcd, y_diff / gcd)
            }
        }
    }

    pub fn enumerate_slopes_to_positions(&self, positions: &Vec<Pos>) -> HashSet<Slope> {
        let mut slopes = HashSet::from_iter(positions.iter().map(|p| self.slope_to(p)).collect::<Vec<Slope>>());
        slopes.remove(&(0, 0)); //remove slope with itself
        slopes
    }
}

//just trying to learn traits
trait DistFrom {
    fn dist_from(self, dest: usize) -> usize;
}

impl DistFrom for usize {
    #[inline]
    fn dist_from(self, dest: usize) -> usize {
        self.max(dest) - self.min(dest)
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<MapLoc>> {
    let line_to_maploc_vec: fn(&str) -> Vec<MapLoc> =
        |line| line
            .chars().map(|c| MapLoc::from(c)).collect();

    input.lines().map(line_to_maploc_vec).collect()
}

fn get_asteroid_positions(asteroid_map: &Vec<Vec<MapLoc>>) -> Vec<Pos> {
    let mut asteroid_positions = Vec::new();

    for (y, row) in asteroid_map.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if loc == &MapLoc::Asteroid {
                asteroid_positions.push(Pos {x, y});
            }
        }
    }

    asteroid_positions
}




#[aoc(day10, part1)]
pub fn part1(asteroid_map: &Vec<Vec<MapLoc>>) -> usize {
    let asteroid_positions = get_asteroid_positions(asteroid_map);
    let mut pos_to_number_seen: HashMap<Pos, usize> = HashMap::new();
    for ast_pos in &asteroid_positions {
        let slopes = ast_pos.enumerate_slopes_to_positions(&asteroid_positions);
        pos_to_number_seen.insert(ast_pos.clone(), slopes.len());
    }

    *pos_to_number_seen.values().max().unwrap()
}
