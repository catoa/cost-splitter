use anyhow::Result;
use clap::{Parser, Subcommand};
use splitter::{charges, ocr};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "splitter")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Charge {
        #[arg(short, long)]
        names: String,
    },

    #[command(arg_required_else_help = true)]
    Ocr {
        #[arg(short, long)]
        receipt_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Charge { names }) => charges::process_individual_charges(names),
        Some(Commands::Ocr { receipt_path }) => ocr::process_receipt(receipt_path).await,
        None => panic!("Unknown CLI command supplied"),
    }
}
