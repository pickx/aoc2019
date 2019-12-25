use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;

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

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {

    let parse_elem = |elem_desc: &str| {
        let (quant_str, elem_name) = elem_desc.split_whitespace().collect_tuple().unwrap();
        let quantity: usize = quant_str.parse().unwrap();
        Element { name: elem_name.to_string(), quantity }
    };

    let mut reactions = Vec::new();

    for line in input.lines() {
        const LHS_RHS_SEPARATOR: &str = " => ";
        const LHS_DELIMITER: &str = ", ";

        let lhs_end = line.find(LHS_RHS_SEPARATOR).unwrap();
        let (lhs, tail) = line.split_at(lhs_end);
        let (_, rhs) = tail.split_at(LHS_RHS_SEPARATOR.len());

        let reaction_inputs: Vec<Element> =  lhs
            .split(LHS_DELIMITER)
            .map(|elem_desc| parse_elem(elem_desc))
            .collect();

        let reaction_output = parse_elem(rhs);

        reactions.push( Reaction { inputs: reaction_inputs, output: reaction_output } );

    }

    reactions
}

fn find_requirements(output_to_reaction: &HashMap<String, Reaction>, requirements: Vec<Element>, base_requirements: &mut Vec<Element>) {

    let mut requirements = requirements;
    let mut inventory: HashMap<String, usize> = HashMap::new();

    while let Some(req) = requirements.pop() {
        let mut next_requirements = Vec::new();
        let req_quantity = req.quantity;

        if let Some(reaction) = output_to_reaction.get(&req.name) {

            let quantity_this_reaction_makes = reaction.output.quantity;

            let mut multiplier = 1;
            while multiplier * quantity_this_reaction_makes < req_quantity {
                multiplier += 1; //no need to do something more efficient here
            }

            for inp in &reaction.inputs {
                let mut inp_amount_needed = inp.quantity * multiplier;

                let amount_we_have_in_inventory = inventory
                    .entry(inp.name.clone())
                    .or_insert(0);

                if *amount_we_have_in_inventory >= inp_amount_needed { //then we don't need to add to requirements
                    *amount_we_have_in_inventory -= inp_amount_needed;
                }

                else {
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


        else {
            base_requirements.push(req);
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

    let requirements: Vec<Element> = vec![target_element];
    let mut base_requirements = Vec::new();

    find_requirements(&output_to_reaction, requirements, &mut base_requirements);

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