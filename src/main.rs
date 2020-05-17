use std::collections::HashMap;
use std::env;
use std::io;
use titlecase::titlecase;

#[derive(Debug)]
struct Charge {
    name: String,
    cost: f64,
}

enum Action {
    Done,
    AddCharge { charge: Charge },
    Invalid { input: String },
}

impl Action {
    fn parse_input(s: &str) -> Self {
        let input: Vec<&str> = s.trim().split_whitespace().into_iter().collect();
        if input.as_slice() == ["Done"] || input.as_slice() == ["done"] {
            Action::Done
        } else if let Some((cost, elements)) = input.split_last() {
            let cost = cost.trim().parse::<f64>().unwrap_or_else(|_| -1.0);
            if cost == -1.0 {
                return Action::Invalid {
                    input: String::from("Cost could not be parsed from string"),
                };
            };
            let name = elements.join(" ");
            let charge = Charge { name, cost };
            println!("{} has been added\n", charge.name);
            Action::AddCharge { charge }
        } else {
            Action::Invalid {
                input: String::from("Supplied string did not match any pattern"),
            }
        }
    }
}

fn print_charge_breakdown(map: HashMap<String, Vec<Charge>>, subtotal: f64) {
    let bill_total = request_bill_total(subtotal);
    let total_fees = bill_total - subtotal;
    for (person, charges) in &map {
        let mut charges_sum: f64 = 0.0;
        println!("\n{}", person);
        println!("==========");
        for charge in charges {
            charges_sum += charge.cost;
            println!("{}: ${:.2}", titlecase(&charge.name), charge.cost);
        }
        println!("Total Charges: ${}", charges_sum);
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

fn process_individual_charges(persons: &[String]) -> (HashMap<String, Vec<Charge>>, f64) {
    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    let mut response = String::new();

    println!("\nEnter the cost of the items for each person");
    println!("Type 'Done' when are you finished entering items for a person");
    for person in persons {
        println!("\nEntering items for {}", person);
        loop {
            io::stdin()
                .read_line(&mut response)
                .expect("Could not read input string");

            let action = Action::parse_input(&response);

            match action {
                Action::Done => {
                    response.clear();
                    break;
                }
                Action::AddCharge { charge } => {
                    subtotal += charge.cost;
                    charge_map
                        .entry(person.to_string())
                        .or_insert_with(Vec::default)
                        .push(charge)
                }
                Action::Invalid { input } => {
                    println!("{}", input);
                }
            }
            response.clear();
        }
    }
    (charge_map, subtotal)
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let persons: &[String] = &args[1..];
    let (charge_map, subtotal) = process_individual_charges(persons);
    print_charge_breakdown(charge_map, subtotal);
}
