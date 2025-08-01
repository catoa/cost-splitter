use crate::errors::Result;
use crate::io::{InputReader, OutputWriter};
use crate::utils::{get_values_by_delim, read_input, read_from_stdin};
use crate::{actions::InputAction, billing};
use colored::Colorize;
use std::collections::HashMap;
use titlecase::titlecase;

#[derive(Debug, PartialEq, Clone)]
pub struct Charge {
    pub name: String,
    pub cost: f64,
    pub is_assigned: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChargeErrorP;

impl Charge {
    pub fn new(name: String, cost: f64) -> Self {
        Self {
            name,
            cost,
            is_assigned: false,
        }
    }
}

/// Process individual charges with dependency injection for testing
pub fn process_individual_charges_with_io<R: InputReader, W: OutputWriter>(
    names: String,
    reader: &mut R,
    writer: &mut W,
) -> Result<HashMap<String, Vec<Charge>>> {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut input = String::new();
    let persons = get_values_by_delim(names, ",");

    writeln!(writer, "\nEnter the cost of the items for each person")?;
    writeln!(
        writer,
        "For example: {}",
        "\tribeye steak 20"
            .bright_white()
            .on_bright_black()
            .italic()
    )?;
    writeln!(
        writer,
        "Type {} when are you finished entering items for a person",
        "'Done'".red()
    )?;

    for person in persons {
        writeln!(writer, "\nEntering items for {}", person.cyan())?;
        loop {
            input.clear();
            read_input(reader, &mut input)?;

            let action = input.parse::<InputAction>();

            match action {
                Ok(InputAction::Done) => {
                    break;
                }
                Ok(InputAction::AddCharge { charge }) => {
                    writeln!(
                        writer,
                        "{} has been added\n",
                        titlecase(&charge.name).yellow().bold()
                    )?;
                    charge_map
                        .entry(person.to_string())
                        .or_default()
                        .push(charge);
                }
                Ok(InputAction::PrintLastCharge) => {
                    if let Some(charges) = charge_map.get(&person) {
                        if let Some(last_charge) = charges.last() {
                            writeln!(
                                writer,
                                "Last item {} has a cost of {}.",
                                last_charge.name, last_charge.cost
                            )?;
                        } else {
                            writeln!(writer, "{} has no items to view.", person)?;
                        }
                    } else {
                        writeln!(writer, "{} has no items to view.", person)?;
                    }
                }
                Ok(InputAction::DeleteLastCharge) => {
                    if let Some(charges) = charge_map.get_mut(&person) {
                        if let Some(charge) = charges.pop() {
                            writeln!(writer, "{} was deleted.", charge.name)?;
                        } else {
                            writeln!(writer, "There are no items to delete.")?;
                        }
                    } else {
                        writeln!(writer, "There are no items to delete.")?;
                    }
                }
                Ok(InputAction::AssignCharge { .. }) | Ok(InputAction::DeleteByIndex { .. }) => {
                    writeln!(writer, "Unsupported action")?;
                }
                Err(e) => {
                    writeln!(writer, "Error parsing input: {e:?}")?;
                }
            }
        }
    }
    Ok(charge_map)
}

/// Legacy function for backwards compatibility
pub fn process_individual_charges(names: String) -> Result<()> {
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
                        .or_default()
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
    billing::print_charge_breakdown(&mut input, &charge_map, subtotal);
    Ok(())
}
