use aoc_runner_derive::{aoc, aoc_generator};
extern crate num;
use num::Integer;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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

type Slope = (isize, isize);
type Deg = f32;
type CanonDeg = i64;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn slope_to(&self, t: &Pos) -> Slope {
        let x_diff = (self.x as isize) - (t.x as isize);
        let y_diff = (self.y as isize) - (t.y as isize);

        match (x_diff, y_diff) {
            (0, 0) => (0, 0),
            (0, _) => (0, y_diff.signum()),
            (_, 0) => (x_diff.signum(), 0),
            (_, _) => {
                let gcd = x_diff.gcd(&y_diff);
                (x_diff / gcd, y_diff / gcd)
            }
        }
    }

    pub fn enumerate_slopes_to_positions(&self, positions: &Vec<Pos>) -> HashSet<Slope> {
        let mut slopes = HashSet::from_iter(
            positions
                .iter()
                .map(|p| self.slope_to(p))
                .collect::<Vec<Slope>>(),
        );
        slopes.remove(&(0, 0)); //remove slope with itself
        slopes
    }

    fn deg_with(&self, t: &Pos) -> Deg {
        let x = t.x as Deg - self.x as Deg;
        let y = t.y as Deg - self.y as Deg;

        x.atan2(y)
    }
}

//adapted from https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
//we must work with i64 approximations of the f32 degrees, because floating points are not hashable.
//also, f32
fn canonicalize(deg: Deg) -> i64 {
    (deg * 1024.0 * 1024.0).round() as i64
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<MapLoc>> {
    let line_to_maploc_vec: fn(&str) -> Vec<MapLoc> =
        |line| line.chars().map(|c| MapLoc::from(c)).collect();

    input.lines().map(line_to_maploc_vec).collect()
}

fn get_asteroid_positions(asteroid_map: &Vec<Vec<MapLoc>>) -> Vec<Pos> {
    let mut asteroid_positions = Vec::new();

    for (y, row) in asteroid_map.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if loc == &MapLoc::Asteroid {
                asteroid_positions.push(Pos { x, y });
            }
        }
    }

    asteroid_positions
}

//compare by distance from s to t1 and from s to t2
//NOTE: assumes both t1 and t2 have same slope with s
fn dist_cmp_from(s: &Pos, t1: &Pos, t2: &Pos) -> Ordering {
    //first confirm that they have the same slope with s
    //and thus, comparison by distance is sound
    assert_eq!(s.slope_to(t1), s.slope_to(t2));

    //cast to isizes to avoid unsigned subtractions
    let (s_x, s_y) = (s.x as isize, s.y as isize);
    let (t1_x, t1_y) = (t1.x as isize, t1.y as isize);
    let (t2_x, t2_y) = (t2.x as isize, t2.y as isize);

    //no need to apply sqrt since sqrt is strictly linear
    let dist1 = (t1_x - s_x).pow(2) + (t1_y - s_y).pow(2);
    let dist2 = (t2_x - s_x).pow(2) + (t2_y - s_y).pow(2);

    if dist1 == 0 {
        return Ordering::Less;
    };
    if dist2 == 0 {
        return Ordering::Greater;
    };

    match dist1 - dist2 {
        0 => Ordering::Equal,
        d if d < 0 => Ordering::Less,
        _ => Ordering::Greater,
    }
}

fn enumerate_degs_from_laser_pos(
    laser_pos: &Pos,
    asteroid_positions: &[Pos],
) -> HashMap<CanonDeg, Vec<Pos>> {
    let mapper = |pos: &Pos| -> (CanonDeg, Pos) {
        let c_deg = canonicalize(laser_pos.deg_with(pos));
        (c_deg, pos.clone())
    };

    let mut deg_to_pos_vec: HashMap<CanonDeg, Vec<Pos>> = HashMap::new();
    for (c_deg, pos) in asteroid_positions
        .iter()
        .filter(|&p| p != laser_pos)
        .map(mapper)
    {
        let v = deg_to_pos_vec.entry(c_deg).or_default();
        v.push(pos);
    }

    deg_to_pos_vec
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

    //max is 338, at Pos { x: 23, y: 20 }
}

#[aoc(day10, part2)]
pub fn part2(asteroid_map: &Vec<Vec<MapLoc>>) -> usize {
    const MONITORING_STATION: Pos = Pos { x: 23, y: 20 };

    let asteroid_positions = get_asteroid_positions(&asteroid_map);
    let laser_position = MONITORING_STATION;

    let mut deg_to_posvec: HashMap<CanonDeg, Vec<Pos>> =
        enumerate_degs_from_laser_pos(&MONITORING_STATION, &asteroid_positions);

    //pre-sort buckets of same slope so that we get O(1) selection of next asteroid to destroy from this angle.
    //note that we sort by reverse order so that we can pop elements efficiently (from the back)
    for posvec in deg_to_posvec.values_mut() {
        posvec.sort_by(|t1, t2| dist_cmp_from(&laser_position, t1, t2).reverse());
    }

    let degs = {
        let mut d: Vec<CanonDeg> = deg_to_posvec.keys().cloned().collect();

        //note we sort in descending order: d2.cmp(d1) instead of d1.cmp(d2)
        //this is because clockwise order of angles is in decreasing order of atan2 values.
        d.sort_by(|d1, d2| d2.cmp(d1));

        d
    };

    const NUM_ASTEROIDS_TO_DESTROY: usize = 200;
    let mut destroyed = 0;

    for deg in degs.iter().cycle() {
        let v = deg_to_posvec.get_mut(deg).unwrap();
        if let Some(pos) = v.pop() {
            destroyed += 1;

            if destroyed == NUM_ASTEROIDS_TO_DESTROY {
                return (pos.x * 100) + pos.y;
            }
        }
    }

    usize::max_value()
}
