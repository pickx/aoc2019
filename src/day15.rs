use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::*;
use std::collections::HashMap;

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
    Wall,
    Empty,
    OxygenSystem,
}

type Pos = (isize, isize); // (x,y ) position relative to starting point (0, 0)


#[derive(Clone, Copy, Eq, PartialEq)]
enum MoveResult {
    HitWall,
    MoveSuccess,
    FoundOxygenSystem,
}

impl MoveResult {
    fn to_corresponding_tile(self) -> Tile {
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


fn adjacent_positions_from(pos: Pos) -> Vec<(Pos, isize)>{
    let (x, y) = (pos.0, pos.1);

    let positions = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
    let movement_codes = [1_isize, 2, 3, 4];

    positions.iter().cloned()
        .zip(movement_codes.iter().cloned())
        .collect()
}

fn advance_runner_until_output(runner: &mut IntcodeRunner, input: isize) -> MoveResult {
    runner.push_input(input);

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
       mut runner: IntcodeRunner,
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


    for (position, code) in adjacent_positions_from(cur_pos) {
        dfs(maze, distances, position, runner.clone(),code, distance + 1);
    }

}


fn find_maze_and_distances(mem: &[isize]) -> (HashMap<Pos, Tile>, HashMap<Pos, usize>) {
    let starting_position = (0, 0);

    let mut maze: HashMap<Pos, Tile> = HashMap::new();
    maze.insert(starting_position, Tile::Empty);

    let mut distances: HashMap<Pos, usize> = HashMap::new();
    distances.insert(starting_position, 0);

    let mut runner = IntcodeRunner::new(mem);
    runner.set_input_consume_mode(InputMode::SingleInput);

    for (position, code) in adjacent_positions_from(starting_position) {
        dfs(&mut maze, &mut distances, position, runner.clone(),code, 1);
    }

    (maze, distances)
}

fn find_oxygen_spread_times(maze: &HashMap<Pos, Tile>,
                            oxygen_spread_time: &mut HashMap<Pos, usize>,
                            cur_pos: Pos,
                            timestamp: usize) {

    if oxygen_spread_time.contains_key(&cur_pos) { return; }

    oxygen_spread_time.insert(cur_pos, timestamp);

    let keep_searching_from_this_pos = |pos| find_oxygen_spread_times(maze, oxygen_spread_time, pos, timestamp + 1);

    //wow look how terse and functional my code is!!!!
    adjacent_positions_from(cur_pos)
        .iter()
        .map(|&(pos, _)| pos)
        .filter(|pos| maze[pos] == Tile::Empty)
        .for_each(keep_searching_from_this_pos);
}

#[aoc(day15, part1)]
fn part1(mem: &[isize]) -> usize {

    let (maze, distances) = find_maze_and_distances(mem);

    for (pos, dist) in distances {
        if maze[&pos] == Tile::OxygenSystem {
            return dist;
        }
    }

    usize::max_value()
}

#[aoc(day15, part2)]
pub fn part2(mem: &[isize]) -> Option<usize> {

    let (maze, _) = find_maze_and_distances(mem);

    let (&oxygen_system_pos, _) = maze
        .iter()
        .find(|(_, &tile)| tile == Tile::OxygenSystem)
        .expect("oxygen system not found on map");

    let mut oxygen_spread_time: HashMap<Pos, usize> = HashMap::new();

    find_oxygen_spread_times(&maze, &mut oxygen_spread_time, oxygen_system_pos, 0);

    oxygen_spread_time
        .values()
        .max()
        .copied()
}