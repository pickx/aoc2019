use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{IntcodeRunner, Opcode};
use std::io;
use std::io::Error;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|val| val.parse())
        .map(|res| res.unwrap())
        .collect()
}

//if let Opcode::In(_) = next_opcode {
//            let mut user_input = String::new();
//            match io::stdin().read_line(&mut user_input) {
//                Ok(_) => (),
//                Err(e) => panic!(e.to_string()),
//            }
//
//            let mut input = user_input.trim_end().parse::<isize>();
//            while input.is_err() {
//                match io::stdin().read_line(&mut user_input) {
//                    Ok(_) => (),
//                    Err(e) => panic!(e.to_string()),
//                }
//                input = user_input.trim_end().parse::<isize>();
//            }
//
//            runner.push_input(input.unwrap());
//        }

fn is_point_affected(row: usize, col: usize, mem: &[isize]) -> bool {
    let mut runner = IntcodeRunner::new(mem);
    runner.push_input(row as isize);//y pos
    runner.push_input(col as isize); //x pos

    loop {
        let next_opcode = runner.parse_cur_opcode();

        let got_output = runner.exec_opcode(next_opcode);

        if got_output {
            let output = runner.output().unwrap();
            return output != 0;
        }
    }
}

fn affected_points_in_area(area_rows: usize, area_cols: usize, mem: &[isize]) -> Vec<Vec<bool>> {
    let mut area = vec![vec![false; area_cols]; area_rows];

    for row in 0..area_rows {
        for col in 0..area_cols {
            area[row][col] = is_point_affected(row, col, mem);
        }
    }

    area
}

fn _draw(image: &Vec<Vec<bool>>) {
    for row in image {
        for c in row {
            let tile = match c {
                true => '#',
                false => '.',
            };
            print!("{}", tile);
        }
        println!();
    }
}

#[aoc(day19, part1)]
fn part1(mem: &[isize]) -> usize {

    let (area_rows, area_cols) = (50, 50);
    let area = affected_points_in_area(area_rows, area_cols, mem);

    _draw(&area);

    area
        .iter()
        .flatten()
        .filter(|&&b| b)
        .count()
}

fn row_of_beam_bottom(cols_from_origin: usize, mem: &[isize]) -> usize {
    let col = cols_from_origin;
    let mut beam_top = 0;
    while !is_point_affected(beam_top, col, mem) {
        beam_top += 1;
    }

    let mut beam_bottom = beam_top;
    while is_point_affected(beam_bottom, col, mem) {
        beam_bottom += 1;
    }

    beam_bottom -= 1;

    beam_bottom
}

//optimization of above algorithm for sequential calls
//note that this only works for large enough n, probably n > 20 is safe
fn row_of_beam_bottom_with_hint(cols_from_origin: usize, bottom_for_previous_col: usize, mem: &[isize]) -> usize {
    let col = cols_from_origin;

    let mut beam_bottom = bottom_for_previous_col;
    while is_point_affected(beam_bottom, col, mem) {
        beam_bottom += 1;
    }

    beam_bottom -= 1;

    beam_bottom
}



#[aoc(day19, part2)]
fn part2(mem: &[isize]) -> usize {
    //we need to have the bottom left corner fit inside the beam, and the top right.
    //the other 2 corners must also fit due to the shape of the beam, no need to check.

    let square_side = 100;

    let mut prev_bottom = row_of_beam_bottom(square_side - 1, mem);

    for near_col in square_side.. {
        let bottom_row = row_of_beam_bottom_with_hint(near_col, prev_bottom, mem);
        prev_bottom = bottom_row;

        // note that we use (square_side - 1) and not square_side!
        let top_row = bottom_row - (square_side - 1);
        let far_col = near_col + (square_side - 1);

        if is_point_affected(bottom_row, near_col, mem) && is_point_affected(top_row, far_col, mem) {
            //found it
            return (near_col * 10000) + top_row;
        }
    }

    unreachable!()
}

#[test]
fn can_load_mem_from_local_var() {
    let input = "109,424,203,1,21101,11,0,0,1105,1,282,21102,18,1,0,1105,1,259,2102,1,1,221,203,1,21102,1,31,0,1106,0,282,21101,38,0,0,1105,1,259,21001,23,0,2,21201,1,0,3,21101,0,1,1,21101,0,57,0,1105,1,303,1201,1,0,222,20102,1,221,3,20101,0,221,2,21101,259,0,1,21102,80,1,0,1106,0,225,21101,127,0,2,21102,91,1,0,1106,0,303,1201,1,0,223,20102,1,222,4,21101,259,0,3,21101,0,225,2,21102,225,1,1,21102,1,118,0,1106,0,225,21001,222,0,3,21101,0,89,2,21101,133,0,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21101,0,148,0,1105,1,259,2102,1,1,223,21002,221,1,4,21001,222,0,3,21101,0,21,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,195,1,0,106,0,108,20207,1,223,2,20102,1,23,1,21102,1,-1,3,21101,0,214,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1201,-4,0,249,22102,1,-3,1,21201,-2,0,2,22101,0,-1,3,21102,250,1,0,1105,1,225,21202,1,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2106,0,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,22101,0,-2,-2,109,-3,2106,0,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,21201,-2,0,3,21101,0,343,0,1106,0,303,1105,1,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,22101,0,-4,1,21101,384,0,0,1106,0,303,1105,1,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21201,1,0,-4,109,-5,2105,1,0";
    let mem = input_generator(input);
    let (area_rows, area_cols) = (50, 50);
    let area = affected_points_in_area(area_rows, area_cols, &mem);

    let actual = area
        .iter()
        .flatten()
        .filter(|&&b| b)
        .count();

    let expected = 203;
    assert_eq!(actual, expected);
}


#[test]
fn confirm_beam_bottom_works() {
    let input = "109,424,203,1,21101,11,0,0,1105,1,282,21102,18,1,0,1105,1,259,2102,1,1,221,203,1,21102,1,31,0,1106,0,282,21101,38,0,0,1105,1,259,21001,23,0,2,21201,1,0,3,21101,0,1,1,21101,0,57,0,1105,1,303,1201,1,0,222,20102,1,221,3,20101,0,221,2,21101,259,0,1,21102,80,1,0,1106,0,225,21101,127,0,2,21102,91,1,0,1106,0,303,1201,1,0,223,20102,1,222,4,21101,259,0,3,21101,0,225,2,21102,225,1,1,21102,1,118,0,1106,0,225,21001,222,0,3,21101,0,89,2,21101,133,0,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21101,0,148,0,1105,1,259,2102,1,1,223,21002,221,1,4,21001,222,0,3,21101,0,21,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,195,1,0,106,0,108,20207,1,223,2,20102,1,23,1,21102,1,-1,3,21101,0,214,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1201,-4,0,249,22102,1,-3,1,21201,-2,0,2,22101,0,-1,3,21102,250,1,0,1105,1,225,21202,1,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2106,0,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,22101,0,-2,-2,109,-3,2106,0,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,21201,-2,0,3,21101,0,343,0,1106,0,303,1105,1,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,22101,0,-4,1,21101,384,0,0,1106,0,303,1105,1,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21201,1,0,-4,109,-5,2105,1,0";
    let mem = input_generator(input);

    let mut prev_bottom_row = row_of_beam_bottom(4, &mem);
    for col_from_origin in 5..100 {
        let expected_bottom_row = row_of_beam_bottom(col_from_origin, &mem);
        let bottom_with_hint = row_of_beam_bottom_with_hint(col_from_origin, prev_bottom_row, &mem);
        assert_eq!(expected_bottom_row, bottom_with_hint);


        prev_bottom_row = expected_bottom_row;
    }



}