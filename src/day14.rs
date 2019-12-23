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

    while let Some(req) = requirements.pop() {

        let req_quantity = req.quantity;

        if let Some(reaction) = output_to_reaction.get(&req.name) {

            if reaction.inputs[0].name != "ORE" {
                for inp in &reaction.inputs {
                let added_req =
                    Element {
                        name: inp.name.clone(),
                        quantity: inp.quantity * req_quantity,
                    };

                requirements.push(added_req);
            }
            }


            else {
//            let base = Element { name: req.name.clone(), quantity: req_quantity };
            base_requirements.push(req);
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

    let requirements: Vec<Element> = vec![target_element];
    let mut base_requirements = Vec::new();

    find_requirements(&output_to_reaction, requirements, &mut base_requirements);

    base_requirements.sort_by_key(|elem| elem.name.clone());
    for (key, group) in &base_requirements.iter().group_by(|elt| &elt.name) {
        let total: usize = group.map(|elm| elm.quantity).sum();
        println!("Element {} has total quantity {}", key, total);
    }
//    dbg!(base_requirements);

    0

//    base_requirements
//        .iter()
//        .map(|elem| elem.quantity)
//        .sum()
}