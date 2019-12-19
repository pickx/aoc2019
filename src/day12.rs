use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::AddAssign;
use std::collections::HashSet;
use num::integer::lcm;

#[derive(Clone, Debug)]
pub struct V3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl V3 {
    pub fn neg(&self) -> V3 {
        V3 { x: -(self.x), y: -(self.y), z: -(self.z) }
    }
}

#[derive(Debug, Clone)]
pub struct Moon {
    pub pos: V3,
    pub vel: V3,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            pos: V3 { x, y, z },
            vel: V3 { x: 0, y: 0, z: 0 },
        }
    }

    //the gravity change for the first of the two inputs.
    pub fn gravity_change(moon_pos: i32, other_moon_pos: i32) -> i32 {
        //if one has lower velocity than the other, it will attract, if it's larger it will repel.
        match moon_pos.cmp(&other_moon_pos) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    pub fn gravity_with(&self, other: &Moon) -> V3 {

        let x = Moon::gravity_change(self.pos.x, other.pos.x);
        let y = Moon::gravity_change(self.pos.y, other.pos.y);
        let z = Moon::gravity_change(self.pos.z, other.pos.z);

        V3 { x, y, z }
    }

    pub fn add_velocity(&mut self, vel_change: &V3) {
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

#[aoc_generator(day12, part1)]
pub fn input_generator(input: &str) -> Vec<Moon> {

    let line_fmt = |line| scan_fmt!(line, "<x={}, y={}, z={}>", i32, i32, i32);

    input
        .lines()
        .map(line_fmt)
        .map(|scan_res| scan_res.expect("Parse error"))
        .map(|(x, y, z)| Moon::new(x, y, z))
        .collect_vec()

}

#[aoc(day12, part1)]
pub fn day1(planets: &[Moon]) -> i32 {
    const STEPS_TO_SIMULATE: usize = 1000 ;

    let mut planets = planets.to_vec();


    for _ in 0..STEPS_TO_SIMULATE {

        let zero_vel = V3 { x: 0, y: 0, z: 0 };
        let mut vel_changes: Vec<V3> = vec![zero_vel; 4];

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

#[aoc_generator(day12, part2)]
fn input_generator_2(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let (mut x_vec, mut y_vec, mut z_vec) = (Vec::new(), Vec::new(), Vec::new());

    let line_fmt = |line| scan_fmt!(line, "<x={}, y={}, z={}>", i32, i32, i32);
    input
        .lines()
        .map(line_fmt)
        .map(|scan_res| scan_res.expect("Parse error"))
        .for_each(|(x, y, z)| { x_vec.push(x); y_vec.push(y); z_vec.push(z); });

    (x_vec, y_vec, z_vec)

}

pub fn find_cycle_period(v: &[i32], steps_to_simulate: usize) -> usize {
    let mut pos = v.to_vec();

    let ZERO_VEC: Vec<i32> = vec![0_i32; 4];
    let mut vel = ZERO_VEC.clone();

    let mut seen: HashSet<Vec<i32>> = HashSet::new();

    for step in 0..=steps_to_simulate {

        // the check against ZERO_VEC is an optimization.
        // since during the earliest cycle we must be back at the initial state
        // and at the initial state, the velocity was zero, there's no need to
        // check states where velocity isn't 0.
        // in other words, not only are we adding less states to the HashSet,
        // we don't need to hash the entire state (v, vel
        if vel == ZERO_VEC {
            let is_new = seen.insert( pos.clone() );
            if !is_new {
                return step;
            }
        }

        for i in 0..(pos.len()-1) {
            for j in (i+1)..pos.len() {

                let change = Moon::gravity_change(pos[i], pos[j]);

                vel[i] += change;
                vel[j] -= change;

            }
        }

        //apply velocity
        for (v_c, &vel_c) in pos.iter_mut().zip(vel.iter()) {
            *v_c += vel_c;
        }

    }

    panic!("No cycle found. Maybe you should try increasing steps_to_simulate.")

}

// this one is tricky. had to look up hints.
// things to understand here are:
// 1) the vectors of each axis X,Y,Z are independent of each other. therefore they can be simulated independently.
// 2) the period for a cycle for the entire system must be a multiple of all 3 periods (hence lcm is used).
// 3)
#[aoc(day12, part2)]
pub fn day2(axis_vecs: &(Vec<i32>, Vec<i32>, Vec<i32>)) -> usize {
    let x_vec = axis_vecs.0.to_vec();
    let y_vec = axis_vecs.1.to_vec();
    let z_vec = axis_vecs.2.to_vec();

    //unnecessarily large but find_cycle_period returns early so whatever
    const STEPS_TO_SIMULATE: usize = 10_000_000;

    let mut cycle_periods: Vec<usize> = Vec::new();
    for v in vec![&x_vec, &y_vec, &z_vec] {
        cycle_periods.push(find_cycle_period(v, STEPS_TO_SIMULATE));
    }

    lcm(cycle_periods[0], lcm(cycle_periods[1], cycle_periods[2]))
}