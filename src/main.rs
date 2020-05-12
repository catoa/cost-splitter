use std::collections::HashMap;
use std::io;

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

fn print_charge_breakdown(map: HashMap<String, Vec<Charge>>, subtotal: f64, taxes_and_fees: f64) {
    for (person, charges) in &map {
        let charges_sum: f64 = charges.iter().map(|c| c.cost).sum();
        println!("\n{}", person);
        println!("==========");
        println!("Total Charges: {}", charges_sum);
        let pct_of_subtotal = charges_sum / subtotal;
        println!(
            "Percent of Subtotal (Total Charges / Subtotal): {:.5}",
            pct_of_subtotal
        );
        let fees_owed = pct_of_subtotal * taxes_and_fees;
        println!("Taxes and Fees owed: {}", fees_owed);
        let amt_owed = charges_sum + fees_owed;
        println!("Total Owed: {:.2}", amt_owed);
    }
}

fn main() {
    println!("Enter the number of persons");
    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("There was an error receiving the number of persons");

    let num_persons = response.trim().parse::<i64>().unwrap();
    response.clear();

    println!("Number of users: {}\n", num_persons);

    let mut persons: Vec<String> = Vec::new();
    for i in 0..num_persons {
        println!("\nEnter the name of friend #{}", i + 1);

        io::stdin()
            .read_line(&mut response)
            .expect("There was an error reading in the users name");

        persons.push(response.trim().to_string().clone());
        response.clear();
    }

    println!("\nEnter the name and cost of the items for each person");
    println!("Type 'Done' when are you finished entering items for a person");

    let mut charge_map: HashMap<String, Vec<Charge>> = HashMap::new();
    let mut subtotal: f64 = 0.0;
    for person in &persons {
        println!("\nEntering items for {}", person);
        response.clear();
        loop {
            io::stdin()
                .read_line(&mut response)
                .expect("Could not read input string");

            let action = Action::parse_input(&response);

            match action {
                Action::Done => break,
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
    response.clear();
    println!("\nEnter the total taxes, fees, tips, etc.\n");

    io::stdin()
        .read_line(&mut response)
        .expect("Unable to read input string");

    let taxes_and_fees: f64 = response.trim().parse().unwrap();

    print_charge_breakdown(charge_map, subtotal, taxes_and_fees);
}
