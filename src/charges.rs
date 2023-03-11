use crate::actions;
use crate::billing;
use colored::Colorize;
use std::collections::HashMap;

use crate::utils::{get_values_by_delim, read_from_stdin};

#[derive(Debug, PartialEq, Clone)]
pub struct Charge {
    pub name: String,
    pub cost: f64,
    pub is_assigned: bool,
}

pub fn process_individual_charges(names: String) {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    let mut input = String::new();
    let persons = get_values_by_delim(names, ",");

    println!("\nEnter the cost of the items for each person");
    println!(
        "For example: {}",
        "\t    ribeye steak 20    "
            .bright_white()
            .on_bright_black()
            .italic()
    );
    println!(
        "Type {} when are you finished entering items for a person",
        "'Done'".red()
    );
    for person in persons {
        println!("\nEntering items for {}", person.cyan());
        loop {
            read_from_stdin(&mut input, "Could not read input string");

            let action = actions::InputAction::parse(&input);

            actions::handle_input_action(
                action.clone(),
                &mut input,
                &mut charge_map,
                &mut subtotal,
                person.clone(),
            );

            match action {
                actions::InputAction::Done => break,
                _ => continue,
            }
        }
    }
    input.clear();
    billing::print_charge_breakdown(&mut input, &charge_map, subtotal)
}
