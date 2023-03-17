use crate::charges::Charge;
use crate::utils::read_from_stdin;
use colored::Colorize;
use std::collections::HashMap;
use titlecase::titlecase;

fn calculate_bill_total(input: &mut String, subtotal: f64) -> Option<f64> {
    println!("\nEnter the total for the bill.");

    read_from_stdin(input, "Unable to read input string");

    match input.trim().parse() {
        Ok(bill_total) => {
            if bill_total >= subtotal {
                Some(bill_total)
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

pub fn print_charge_breakdown(
    input: &mut String,
    charges: &HashMap<String, Vec<Charge>>,
    subtotal: f64,
) {
    let bill_total = calculate_bill_total(input, subtotal);
    let total_fees = bill_total.unwrap() - subtotal;
    for (person, charges) in charges {
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
        let pct_of_subtotal_string = format!("{:.2?}%", (pct_of_subtotal * 100.0)).red();
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
