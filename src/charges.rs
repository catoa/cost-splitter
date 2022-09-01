use std::collections::HashMap;
use std::io;
use titlecase::titlecase;

#[derive(Debug)]
pub struct Charge {
    pub name: String,
    pub cost: f64,
}

#[derive(Debug)]
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
            println!("{} has been added\n", charge.name);
            Action::AddCharge { charge }
        } else {
            Action::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        }
    }
}

fn print_charge_breakdown(charges: HashMap<String, Vec<Charge>>, subtotal: f64) {
    let bill_total = request_bill_total(subtotal);
    let total_fees = bill_total - subtotal;
    for (person, charges) in &charges {
        let mut charges_sum: f64 = 0.0;
        println!("\n{}", person);
        println!("==========");
        for charge in charges {
            charges_sum += charge.cost;
            println!("{}: ${:.2}", titlecase(&charge.name), charge.cost);
        }
        println!("Total Charges: ${:.2}", charges_sum);
        let pct_of_subtotal = charges_sum / subtotal;
        println!(
            "Percent of Subtotal (Total Charges / Subtotal): {:.2}",
            pct_of_subtotal
        );
        let fees_owed = pct_of_subtotal * total_fees;
        println!("Fees owed: ${:.2}", fees_owed);
        let amt_owed = charges_sum + fees_owed;
        println!("Total Owed: ${:.2}", amt_owed);
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

pub fn process_charges(names: String) {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    let mut response = String::new();
    let persons = gather_names(names);

    println!("\nEnter the cost of the items for each person");
    println!("Type 'Done' when are you finished entering items for a person");
    for person in persons {
        response.clear();
        println!("\nEntering items for {}", person);
        loop {
            io::stdin()
                .read_line(&mut response)
                .expect("Could not read input string");

            let action = Action::parse_input(&response);
            match action {
                Action::Done => {
                    break;
                }
                Action::AddCharge { charge } => {
                    subtotal += charge.cost;

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
                            subtotal -= cost;
                            println!("{} was deleted.", name);
                        }
                        None => println!("There are no items to delete."),
                    };
                }
                Action::Invalid { msg } => {
                    println!("{}", msg);
                    continue;
                }
            }
            response.clear();
        }
    }
    print_charge_breakdown(charge_map, subtotal)
}

fn request_bill_total(subtotal: f64) -> f64 {
    println!("\nEnter the total for the bill.\n");
    let mut response = String::new();
    loop {
        io::stdin()
            .read_line(&mut response)
            .expect("Unable to read input string");

        let bill_total: f64 = response.trim().parse().unwrap();

        if bill_total > subtotal {
            return bill_total;
        } else {
            println!("The amount you entered was less than the subtotal.\nPlease try again.\n");
            response.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gather_names;
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
    fn test_request_bill_total() {}
}
