use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::AddAssign;

#[derive(Clone, Debug)]
pub struct V3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl AddAssign for V3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl V3D {
    pub fn neg(&self) -> V3D{
        V3D { x: -(self.x), y: -(self.y), z: -(self.z) }
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub pos: V3D,
    pub vel: V3D,
}

impl Planet {
    pub fn new(x: i32, y: i32, z: i32) -> Planet {
        Planet {
            pos: V3D { x, y, z },
            vel: V3D { x: 0, y: 0, z: 0 },
        }
    }

    pub fn gravity_with(&self, other: &Planet) -> V3D {

        //if one is smaller than the other, it will attract, if it's larger it will repel.
        let gravity_change: fn(i32, i32) -> i32 = |c1, c2| match c1.cmp(&c2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let x = gravity_change(self.pos.x, other.pos.x);
        let y = gravity_change(self.pos.y, other.pos.y);
        let z = gravity_change(self.pos.z, other.pos.z);

        V3D { x, y, z }
    }

    pub fn add_velocity(&mut self, vel_change: &V3D) {
        self.vel += vel_change.clone();
    }

    pub fn apply_own_velocity(&mut self) {
        self.pos += self.vel.clone();
    }

    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    pub fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Planet> {

    let line_fmt = |line| scan_fmt!(line, "<x={}, y={}, z={}>", i32, i32, i32);

    input
        .lines()
        .map(line_fmt)
        .map(|scan_res| scan_res.expect("Parse error"))
        .map(|(x, y, z)| Planet::new(x, y, z))
        .collect_vec()

}

#[aoc(day12, part1)]
pub fn day1(planets: &[Planet]) -> i32 {
    const STEPS_TO_SIMULATE: usize = 1000 ;

    let mut planets = planets.to_vec();


    for _ in 0..STEPS_TO_SIMULATE {

        let zero_vel = V3D { x: 0, y: 0, z: 0 };
        let mut vel_changes: Vec<V3D> = vec![zero_vel; 4];

        for (combination, indexes) in planets.iter().combinations(2).zip((0..4).combinations(2)) {
            let gravity = combination[0].gravity_with(combination[1]);

            vel_changes[indexes[0]] += gravity.clone();
            vel_changes[indexes[1]] += gravity.neg().clone();


        }

        for (planet, vel_change) in planets.iter_mut().zip(vel_changes.iter()) {
            planet.add_velocity(&vel_change);
            planet.apply_own_velocity();
        }

    }


    planets
        .iter()
        .map(|planet| planet.total_energy())
        .sum()
}

#[aoc(day12, part2)]
pub fn day2(planets: &[Planet]) -> i32 {
    0
}