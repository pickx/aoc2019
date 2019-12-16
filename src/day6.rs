use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn line_splitter(line: &str) -> Option<(String, String)> {
    line.split(')')
        .map(|s| s.to_string())
        .collect_tuple()
}

fn dfs(
    planet_depths: &mut HashMap<String, usize>,
    children_of: &HashMap<String, Vec<String>>,
    start_point: &str,
    depth: usize,
) {
    let mut depth = depth;
    if let Some(children) = children_of.get(start_point) {
        depth += 1;
        for child in children {
            planet_depths.insert(child.clone(), depth);
            dfs(planet_depths, children_of, child, depth);
        }
    }
}

fn ancestors(parent_of: &HashMap<String, String>, start_point: &str) -> Vec<String> {
    //avoiding the cost of reverse by building output backwards
    let mut ancestors = VecDeque::new();

    let mut child = start_point.to_string();

    while let Some(parent) = parent_of.get(child.as_str()) {
        ancestors.push_front(child);
        child = parent.to_string();
    }

    ancestors.into()
}

fn find_distance(s1: &[String], s2: &[String]) -> usize {
    let mut iter1 = s1.iter().peekable();
    let mut iter2 = s2.iter().peekable();

    while let (Some(nxt1), Some(nxt2)) = (iter1.peek(), iter2.peek()) {
        if nxt1 != nxt2 {
            break;
        }

        iter1.next();
        iter2.next();
    }

    iter1.count() + iter2.count() - 2
}

#[aoc_generator(day6, part1)]
pub fn input_generator_part1(input: &str) -> HashMap<String, Vec<String>> {
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();

    for (parent, child) in input.lines().map(|line| line_splitter(line).unwrap()) {
        children_of.entry(parent).or_insert(vec![]).push(child);
    }

    children_of
}

#[aoc_generator(day6, part2)]
pub fn input_generator_part2(input: &str) -> HashMap<String, String> {
    HashMap::from_iter(
        input
            .lines()
            .map(|line| line_splitter(line).unwrap())
            .map(|(parent, child)| (child, parent)),
    )
}

#[aoc(day6, part1)]
pub fn part1(children_of: &HashMap<String, Vec<String>>) -> usize {
    let mut planet_depths: HashMap<String, usize> = HashMap::new();
    dfs(&mut planet_depths, children_of, "COM", 0);

    planet_depths.values().sum()
}

#[aoc(day6, part2)]
pub fn part2(parent_of: &HashMap<String, String>) -> usize {
    let you_ancestors = ancestors(parent_of, "YOU");
    let san_ancestors = ancestors(parent_of, "SAN");

    find_distance(&you_ancestors, &san_ancestors)
}
