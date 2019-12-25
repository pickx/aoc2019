use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::cmp::min;

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

fn find_requirements(output_to_reaction: &HashMap<String, Reaction>,
                     target_element: Element,
                     base_requirements: &mut Vec<Element>) {

    const BASE_ELEMENT: &str = "ORE";

    let mut requirements = VecDeque::new();
    requirements.push_back(target_element);

    let mut reserve: HashMap<String, usize> = HashMap::new();

    while let Some(req) = requirements.pop_front() {

        let reaction = output_to_reaction
            .get(&req.name)
            .expect("Element has no reaction that makes it.");

        let quantity_we_need_to_make = req.quantity;
        let quantity_in_reserve = reserve.entry(req.name.clone()).or_insert(0);

        let quantity_we_can_take_from_reserve = min(*quantity_in_reserve, quantity_we_need_to_make);

        *quantity_in_reserve -= quantity_we_can_take_from_reserve;

        let quantity_left_to_make = quantity_we_need_to_make - quantity_we_can_take_from_reserve;

        let quantity_reaction_makes = reaction.output.quantity;
        let multiplier = 1 + (quantity_left_to_make / (quantity_reaction_makes + 1));
//        quantity_left_to_make    4
//        quantity_reaction_makes  4




        let mut multiplier = 1;
        while multiplier * quantity_reaction_makes < required_quantity {
            multiplier += 1; //no need to do something more efficient here
        }

        for inp in &reaction.inputs {
            let mut inp_amount_needed = inp.quantity * multiplier;

            let amount_we_have_in_inventory = inventory
                .entry(inp.name.clone())
                .or_insert(0);


            if *amount_we_have_in_inventory >= inp_amount_needed { //then we don't need to add to requirements
                *amount_we_have_in_inventory -= inp_amount_needed;
            } else {
                inp_amount_needed -= *amount_we_have_in_inventory;
                *amount_we_have_in_inventory = 0;

                let added_req =
                    Element {
                        name: inp.name.clone(),
                        quantity: inp_amount_needed,
                    };

                *amount_we_have_in_inventory = inp_amount_needed;

                requirements.push(added_req);
            }

            if *amount_we_have_in_inventory == 0 {
                inventory.remove(&inp.name);
            }
        }
    }
}

#[aoc(day14, part1)]
pub fn day1(reactions: &[Reaction]) -> usize {

    let name_and_reaction = reactions
        .iter()
        .map(|reaction| (reaction.output.name.clone(), reaction.clone()) );

    let output_to_reaction: HashMap<String, Reaction> = HashMap::from_iter(name_and_reaction);


    let target_element = Element { name: "FUEL".to_string(), quantity: 1 };

    let mut base_requirements = Vec::new();

    find_requirements(&output_to_reaction, target_element, &mut base_requirements);

//    base_requirements.sort_by_key(|elem| elem.name.clone());
//    for (key, group) in &base_requirements.iter().group_by(|elt| &elt.name) {
//        let total: usize = group.map(|elm| elm.quantity).sum();
//        println!("Element {} has total quantity {}", key, total);
//    }
//    dbg!(base_requirements);

//    0

    base_requirements
        .iter()
        .map(|elem| elem.quantity)
        .sum()
}