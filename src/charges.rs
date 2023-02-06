use crate::actions;
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
            let cost_string = format!("${:.2}", charge.cost).green().bold();
            println!(
                "{}: {}",
                titlecase(&charge.name).bright_white().bold(),
                cost_string
            )
        }
        let total_charges_string = format!("${:.2}", charges_sum).green();
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

pub fn process_charges(names: String) {
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
    print_charge_breakdown(&mut input, charge_map, subtotal)
}

fn calculate_bill_total(input: &mut String, subtotal: f64) -> Option<f64> {
    println!("\nEnter the total for the bill.");

    read_from_stdin(input, "Unable to read input string");

    match input.trim().parse() {
        Ok(bill_total) => {
            if bill_total >= subtotal {
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
