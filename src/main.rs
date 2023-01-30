use clap::Parser;

use cost_splitter::charges;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, multiple_values = true)]
    names: String,
}

fn main() {
    let args = Args::parse();
    charges::process_charges(args.names);
}
