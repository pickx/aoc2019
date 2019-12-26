use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::cmp::min;
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Element {
    name: String,
    quantity: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Reaction {
    inputs: Vec<Element>,
    output: Element,
}

fn parse_elem(elem_desc: &str) -> Element {
    let mut iter = elem_desc.split_whitespace();

    let quantity: usize = iter
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse quantity");

    let name = iter
        .next()
        .unwrap()
        .to_string();

    Element { name, quantity }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {

    const LHS_RHS_SEPARATOR: &str = " => ";
    const LHS_DELIMITER: &str = ", ";

    let mut reactions = Vec::new();

    for line in input.lines() {

        let mut iter = line.split(LHS_RHS_SEPARATOR);

        let reaction_inputs: Vec<Element> = iter
            .next()
            .expect("No lhs")
            .split(LHS_DELIMITER)
            .map(|elem_desc| parse_elem(elem_desc))
            .collect();


        let reaction_output = parse_elem(iter.next().expect("No rhs"));

        reactions.push( Reaction { inputs: reaction_inputs, output: reaction_output } );

    }

    reactions
}

fn check_reserve(quantity_in_reserve: &mut usize, quantity_we_need_to_make: usize) -> usize {

    let quantity_we_can_take_from_reserve = min(*quantity_in_reserve, quantity_we_need_to_make);

    *quantity_in_reserve -= quantity_we_can_take_from_reserve;

    let quantity_left_to_make = quantity_we_need_to_make - quantity_we_can_take_from_reserve;

    quantity_left_to_make
}

fn find_base_amount_needed(output_to_reaction: &HashMap<String, Reaction>, target_quantity: usize) -> usize {


    let target_element = Element { name: "FUEL".to_string(), quantity: target_quantity };
    const BASE_ELEMENT: &str = "ORE";

    let mut requirements = VecDeque::new();
    requirements.push_back(target_element);

    let mut reserve: HashMap<String, usize> = HashMap::new();

    let mut base_element_amount_created = 0;

    while let Some(req) = requirements.pop_front() {

        let reaction = output_to_reaction
            .get(&req.name)
            .expect("Element has no reaction that makes it.");


        // this is all done in a block to not pollute the namespace with all those lets,
        // and to make sure the mutable borrow of quantity_in_reserve is properly dropped.
        // probably none of this is necessary and I'm just over-complicating things.

        let mut quantity_in_reserve = reserve
            .entry(req.name.clone())
            .or_insert(0);

        let quantity_to_make = check_reserve(&mut quantity_in_reserve, req.quantity);

        if quantity_to_make == 0 {
            continue;
        }

        let quantity_reaction_makes = reaction.output.quantity;

        let multiplier = {
            let mut multiplier = quantity_to_make / quantity_reaction_makes;

            while multiplier * quantity_reaction_makes < quantity_to_make {
                multiplier += 1;
            }

            multiplier
        };

        *quantity_in_reserve += (multiplier * quantity_reaction_makes) - quantity_to_make;

        for inp in &reaction.inputs {

            let quantity_we_need_to_make = inp.quantity * multiplier;

            if inp.name == BASE_ELEMENT {
                base_element_amount_created += quantity_we_need_to_make;
            }
            else {
                let mut quantity_in_reserve = reserve
                    .entry(inp.name.clone())
                    .or_insert(0);

                let quantity_to_make = check_reserve(&mut quantity_in_reserve, quantity_we_need_to_make);

                if quantity_to_make > 0 {
                    let added_requirement = Element { name: inp.name.clone(), quantity: quantity_to_make };
                    requirements.push_back(added_requirement);
                }
            }
        }
    }

    base_element_amount_created
}

fn create_output_to_reaction(reactions: &[Reaction]) -> HashMap<String, Reaction> {
    let name_iter = reactions
        .iter()
        .cloned()
        .map(|reaction| reaction.output.name);
    let reaction_iter = reactions
        .iter()
        .cloned();

    let output_to_reaction: HashMap<String, Reaction> = HashMap::from_iter(
        name_iter.zip(reaction_iter)
    );

    output_to_reaction
}

#[aoc(day14, part1)]
pub fn day1(reactions: &[Reaction]) -> usize {

    let output_to_reaction = create_output_to_reaction(reactions);

    find_base_amount_needed(&output_to_reaction, 1)

}

#[aoc(day14, part2)]
fn part2(reactions: &[Reaction]) -> usize {

    let ore_amount: usize = 1_000_000_000_000;

    let output_to_reaction = create_output_to_reaction(reactions);

//    we do a binary search in the interval [x, SEARCH_FACTOR * x]
//    such that x <= maximum_amount_of_fuel_we_can_make,
//    but SEARCH_FACTOR * x > maximum_amount_of_fuel_we_can_make.
    let mut hi = 1;

    //did a few benchmarks, this factor consistently gives good performance
    const SEARCH_FACTOR: usize = 11;

    while find_base_amount_needed(&output_to_reaction, hi) < ore_amount {
        hi *= SEARCH_FACTOR;
    }

    let mut lo = hi / SEARCH_FACTOR;

    loop {
        let mid = (lo + hi) / 2;

        if find_base_amount_needed(&output_to_reaction, mid) > ore_amount {
            hi = mid - 1;
        }

        else {
            if find_base_amount_needed(&output_to_reaction, mid + 1) > ore_amount {
                break mid //equivalent to "return mid;" but with more swag
            }

            lo = mid + 1;

        }
    }


}