use clap::Parser;

use cost_splitter::charges;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, multiple_values = true)]
    names: String,
}

fn main() {
    let args = Cli::parse();
    charges::process_charges(args.names);
}
