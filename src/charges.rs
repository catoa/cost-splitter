use colored::Colorize;
use std::collections::HashMap;
use std::io;
use titlecase::titlecase;

#[derive(Debug, PartialEq, Clone)]
pub struct Charge {
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Done,
    AddCharge { charge: Charge },
    PrintLastCharge,
    DeleteLastCharge,
    Invalid { msg: String },
}

impl Action {
    pub fn parse_input(s: &str) -> Self {
        let input: Vec<&str> = s.trim().split_whitespace().into_iter().collect();
        if input.as_slice() == ["Done"] || input.as_slice() == ["done"] {
            Action::Done
        } else if input.as_slice() == ["Last"] || input.as_slice() == ["last"] {
            Action::PrintLastCharge
        } else if input.as_slice() == ["Delete"] || input.as_slice() == ["delete"] {
            Action::DeleteLastCharge
        } else if let Some((cost, elements)) = input.split_last() {
            let cost = cost.trim().parse::<f64>().unwrap_or_else(|_| -1.0);
            if cost == -1.0 {
                return Action::Invalid {
                    msg: String::from("Cost could not be parsed from string"),
                };
            };
            let name = elements.join(" ");
            if name == "" {
                return Action::Invalid {
                    msg: String::from("Could not parse line item because name was not supplied"),
                };
            }
            let charge = Charge { name, cost };
            println!(
                "{} has been added\n",
                titlecase(&charge.name).yellow().bold()
            );
            Action::AddCharge { charge }
        } else {
            Action::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        }
    }
}

fn print_charge_breakdown(
    input: &mut String,
    charges: HashMap<String, Vec<Charge>>,
    subtotal: f64,
) {
    let bill_total = calculate_bill_total(input, subtotal);
    let total_fees = bill_total.unwrap() - subtotal;
    for (person, charges) in &charges {
        let mut charges_sum: f64 = 0.0;
        println!("\n{}", person.cyan());
        println!("{}", "==========".blue());
        for charge in charges {
            charges_sum += charge.cost;
            let cost_string = format!("${:.2}", charge.cost.to_string()).green().bold();
            println!(
                "{}: {}",
                titlecase(&charge.name).bright_white().bold(),
                cost_string
            )
        }
        let total_charges_string = format!("${}", charges_sum.to_string()).green();
        println!("Total Charges: {}", total_charges_string);
        let pct_of_subtotal = charges_sum / subtotal;
        let pct_of_subtotal_string = format!("{}%", (pct_of_subtotal * 100.0).to_string()).red();
        if pct_of_subtotal < 1.0 {
            println!(
                "Percent of Subtotal (Total Charges / Subtotal): {}",
                pct_of_subtotal_string
            );
        }
        let fees_owed = pct_of_subtotal * total_fees;
        let fees_owed_string = format!("${:.2}", fees_owed).red();
        println!("Fees owed: {}", fees_owed_string);
        let amt_owed = charges_sum + fees_owed;

        let amt_owed_string = format!("${:.2}", amt_owed).green().bold();
        println!("Total Owed: {}", amt_owed_string);
    }
}

pub fn gather_names(names: String) -> Vec<String> {
    names
        .split(",")
        .filter_map(|name| {
            if name.is_empty() {
                return None;
            }
            Some(name.to_owned())
        })
        .collect::<Vec<String>>()
}

fn read_from_stdin(msg: &mut String, error_msg: String) {
    io::stdin().read_line(msg).expect(&error_msg);
}

fn handle_input_action(
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

pub fn process_charges(names: String) {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    let mut input = String::new();
    let persons = gather_names(names);

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
            read_from_stdin(&mut input, String::from("Could not read input string"));

            let action = Action::parse_input(&input);

            handle_input_action(
                action.clone(),
                &mut input,
                &mut charge_map,
                &mut subtotal,
                person.clone(),
            );

            match action {
                Action::Done => break,
                _ => continue,
            }
        }
    }
    input.clear();
    print_charge_breakdown(&mut input, charge_map, subtotal)
}

fn calculate_bill_total(input: &mut String, subtotal: f64) -> Option<f64> {
    println!("\nEnter the total for the bill.");

    read_from_stdin(input, String::from("Unable to read input string"));

    match input.trim().parse() {
        Ok(bill_total) => {
            if bill_total >= subtotal {
                println!("we in here");
                return Some(bill_total);
            } else {
                println!("Amount was less than subtotal. Please try again.");
                calculate_bill_total(input, subtotal)
            }
        }
        Err(_) => {
            println!("Could not parse input, {}", input);
            calculate_bill_total(input, subtotal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{gather_names, Action, Charge};

    #[test]
    fn test_split_names_handles_leading_comma() {
        assert_eq!(
            gather_names(String::from(",Anthony,Caroline")),
            vec!["Anthony", "Caroline"]
        );
    }
    #[test]
    fn test_split_names_handles_trailing_comma() {
        assert_eq!(
            gather_names(String::from("Anthony,Caroline,")),
            vec!["Anthony", "Caroline"]
        );
    }

    #[test]
    fn test_split_names_handles_leading_and_trailing_comma() {
        assert_eq!(
            gather_names(String::from(",Anthony,Caroline,")),
            vec!["Anthony", "Caroline"]
        );
    }

    #[test]
    fn test_action_parse_input() {
        assert_eq!(Action::parse_input("done"), Action::Done);
        assert_eq!(Action::parse_input("Done"), Action::Done);
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
        assert_eq!(Action::parse_input("Last"), Action::PrintLastCharge);
        assert_eq!(Action::parse_input("delete"), Action::DeleteLastCharge);
        assert_eq!(Action::parse_input("Delete"), Action::DeleteLastCharge);

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
