use std::env;

mod lib;

use lib::{print_charge_breakdown, process_individual_charges};

fn main() {
    let args: Vec<String> = env::args().collect();
    let persons: &[String] = &args[1..];
    let (charge_map, subtotal) = process_individual_charges(persons);
    print_charge_breakdown(charge_map, subtotal);
}
