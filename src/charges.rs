use crate::{actions::InputAction, billing};
use colored::Colorize;
use std::collections::HashMap;
use titlecase::titlecase;

use crate::utils::{get_values_by_delim, read_from_stdin};

#[derive(Debug, PartialEq, Clone)]
pub struct Charge {
    pub name: String,
    pub cost: f64,
    pub is_assigned: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChargeError;

impl Charge {
    pub fn new(name: String, cost: f64) -> Self {
        Self {
            name,
            cost,
            is_assigned: false,
        }
    }
}

pub fn process_individual_charges(names: String) {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    let mut input = String::new();
    let persons = get_values_by_delim(names, ",");

    println!("\nEnter the cost of the items for each person");
    println!(
        "For example: {}",
        "\tribeye steak 20"
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

            let action = input.parse::<InputAction>();

            match action {
                Ok(InputAction::Done) => {
                    input.clear();
                    break;
                }
                Ok(InputAction::AddCharge { charge }) => {
                    subtotal += charge.cost;

                    println!(
                        "{} has been added\n",
                        titlecase(&charge.name).yellow().bold()
                    );
                    charge_map
                        .entry(person.to_string())
                        .or_insert_with(Vec::default)
                        .push(charge)
                }
                Ok(InputAction::PrintLastCharge) => {
                    let person_charges = charge_map.get_mut(&person).or(None);

                    match person_charges {
                        Some(charges) => {
                            let last_charge = charges.last().unwrap();
                            println!(
                                "Last item {} has a cost of {}.",
                                last_charge.name, last_charge.cost
                            );
                        }
                        None => println!("{} has no items to view.", person),
                    };
                }
                Ok(InputAction::DeleteLastCharge) => {
                    let person_charges = charge_map.get_mut(&person).or(None);

                    match person_charges {
                        Some(charges) => {
                            let deleted_charge = charges.pop();
                            match deleted_charge {
                                Some(charge) => {
                                    subtotal -= charge.cost;
                                    println!("{} was deleted.", charge.name);
                                }
                                None => println!("There are no items to delete."),
                            }
                        }
                        None => println!("There are no items to delete."),
                    };
                }
                Ok(InputAction::AssignCharge {
                    name: _,
                    indices: _,
                })
                | Ok(InputAction::DeleteByIndex { indices: _ }) => {
                    println!("Unsupported action")
                }
                Err(e) => println!("Error parsing input: {e:?}"),
            }
            input.clear();
        }
    }
    input.clear();
    billing::print_charge_breakdown(&mut input, &charge_map, subtotal)
}
