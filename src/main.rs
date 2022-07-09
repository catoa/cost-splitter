use clap::Parser;

mod lib;

use lib::{print_charge_breakdown, process_individual_charges};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Name of the person to greet
    names: String,
}

fn main() {
    let args = Args::parse();
    let persons = args
        .names
        .split(",")
        .map(|name| name.to_owned())
        .collect::<Vec<String>>();
    let (charge_map, subtotal) = process_individual_charges(&persons);
    print_charge_breakdown(charge_map, subtotal);
}
