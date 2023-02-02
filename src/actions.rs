use crate::charges::Charge;
use colored::Colorize;
use std::collections::HashMap;
use titlecase::titlecase;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Done,
    AddCharge { charge: Charge },
    PrintLastCharge,
    DeleteLastCharge,
    Invalid { msg: String },
}

impl Action {
    pub fn parse_input(input: &str) -> Self {
        match input {
            "done" => Action::Done,
            "delete" => Action::DeleteLastCharge,
            "last" => Action::PrintLastCharge,
            _ => {
                if let Some((cost, elements)) = input
                    .trim()
                    .split_whitespace()
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .split_last()
                {
                    let cost = cost.trim().parse::<f64>().unwrap_or_else(|_| -1.0);
                    if cost == -1.0 {
                        return Action::Invalid {
                            msg: String::from("Cost could not be parsed from string"),
                        };
                    };
                    let name = elements.join(" ");
                    if name == "" {
                        return Action::Invalid {
                            msg: String::from(
                                "Could not parse line item because name was not supplied",
                            ),
                        };
                    }
                    let charge = Charge { name, cost };
                    return Action::AddCharge { charge };
                } else {
                    return Action::Invalid {
                        msg: String::from("Supplied string did not match any pattern"),
                    };
                }
            }
        }
    }
}

pub fn handle_input_action(
    action: Action,
    input: &mut String,
    charge_map: &mut HashMap<String, Vec<Charge>>,
    subtotal: &mut f64,
    person: String,
) {
    match action {
        Action::Done => {
            input.clear();
        }
        Action::AddCharge { charge } => {
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
        Action::PrintLastCharge => {
            let charges = charge_map.get_mut(&person).unwrap();
            match charges.last() {
                Some(Charge { name, cost }) => {
                    println!("Last item {} has a cost of {}.", name, cost);
                }
                None => println!("{} has no items to view.", person),
            };
        }
        Action::DeleteLastCharge => {
            let charges = charge_map.get_mut(&person).unwrap();
            match charges.pop() {
                Some(Charge { name, cost }) => {
                    *subtotal -= cost;
                    println!("{} was deleted.", name);
                }
                None => println!("There are no items to delete."),
            };
        }
        Action::Invalid { ref msg } => {
            println!("{}", msg);
        }
    }
    input.clear();
}

#[cfg(test)]
mod tests {
    use super::{Action, Charge};

    #[test]
    fn test_action_parse_input() {
        assert_eq!(Action::parse_input("done"), Action::Done);
        assert_eq!(
            Action::parse_input(" "),
            Action::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        );
        assert_eq!(
            Action::parse_input(""),
            Action::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        );
        assert_eq!(Action::parse_input("last"), Action::PrintLastCharge);
        assert_eq!(Action::parse_input("delete"), Action::DeleteLastCharge);

        assert_eq!(
            Action::parse_input("Steak Sandwich 20"),
            Action::AddCharge {
                charge: Charge {
                    name: String::from("Steak Sandwich"),
                    cost: 20.00
                }
            }
        );

        assert_eq!(
            Action::parse_input("social smoker 8"),
            Action::AddCharge {
                charge: Charge {
                    name: String::from("social smoker"),
                    cost: 8.00
                }
            }
        );

        assert_eq!(
            Action::parse_input("we don't have a price"),
            Action::Invalid {
                msg: String::from("Cost could not be parsed from string"),
            }
        );

        assert_eq!(
            Action::parse_input("48"),
            Action::Invalid {
                msg: String::from("Could not parse line item because name was not supplied"),
            }
        );
    }
}
