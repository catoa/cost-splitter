use crate::charges::Charge;
use crate::utils::get_values_by_delim;
use colored::Colorize;
use std::collections::HashMap;
use std::vec::Vec;
use titlecase::titlecase;

#[derive(Debug, PartialEq, Clone)]
pub enum InputAction {
    AddCharge { charge: Charge },
    AssignCharge { name: String, indices: Vec<String> },
    PrintLastCharge,
    DeleteLastCharge,
    DeleteByIndex { indices: Vec<String> },
    Done,
    Invalid { msg: String },
}

impl InputAction {
    pub fn parse(input: &str) -> InputAction {
        match input.trim() {
            "done" | "Done" => InputAction::Done,
            "delete" | "Delete" => InputAction::DeleteLastCharge,
            "last" | "Last" => InputAction::PrintLastCharge,
            _ => {
                if let Some((val, text)) =
                    input.split_whitespace().collect::<Vec<&str>>().split_last()
                {
                    let name = text.join(" ");
                    if name.is_empty() {
                        return InputAction::Invalid {
                            msg: String::from(
                                "Could not parse line item because name was not supplied",
                            ),
                        };
                    }
                    match name.to_lowercase().as_ref() {
                        "delete" => {
                            let val = get_values_by_delim(val.to_string(), ",");
                            InputAction::DeleteByIndex { indices: val }
                        }
                        _ => {
                            if name.starts_with("assign") {
                                let val = get_values_by_delim(val.to_string(), ",");
                                let mut name_iter = name.split_whitespace();

                                name_iter.next();
                                let person = name_iter.next().unwrap();
                                InputAction::AssignCharge {
                                    name: person.to_owned(),
                                    indices: val,
                                }
                            } else {
                                let val = val.parse::<f64>().unwrap_or_else(|_| -1.0);
                                if val == -1.0 {
                                    return InputAction::Invalid {
                                        msg: String::from("Value could not be parsed from string"),
                                    };
                                };
                                let charge = Charge {
                                    name,
                                    cost: val,
                                    is_assigned: false,
                                };
                                InputAction::AddCharge { charge }
                            }
                        }
                    }
                } else {
                    InputAction::Invalid {
                        msg: String::from("Supplied string did not match any pattern"),
                    }
                }
            }
        }
    }
}

pub fn handle_input_action(
    action: InputAction,
    input: &mut String,
    charge_map: &mut HashMap<String, Vec<Charge>>,
    subtotal: &mut f64,
    person: String,
) {
    match action {
        InputAction::Done => {
            input.clear();
        }
        InputAction::AddCharge { charge } => {
            *subtotal += charge.cost;

            println!(
                "{} has been added\n",
                titlecase(&charge.name).yellow().bold()
            );
            charge_map
                .entry(person.to_string())
                .or_insert_with(Vec::default)
                .push(charge)
        }
        InputAction::PrintLastCharge => {
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
        InputAction::DeleteLastCharge => {
            let person_charges = charge_map.get_mut(&person).or(None);

            match person_charges {
                Some(charges) => {
                    let deleted_charge = charges.pop().unwrap();
                    *subtotal -= deleted_charge.cost;
                    println!("{} was deleted.", deleted_charge.name);
                }
                None => println!("There are no items to delete."),
            };
        }
        InputAction::Invalid { msg } => {
            println!("{}", msg);
        }
        _ => {
            println!("Unsupported operation");
        }
    }
    input.clear();
}
