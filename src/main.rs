mod cli;
mod common_ports;
mod dns;
mod error;
mod modules;
mod ports;

use std::env;

use clap::{self, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all modules
    Modules,
    /// Runs a TCP Connect scan against the target
    Scan {
        /// Host to scan
        target: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    unsafe { env::set_var("RUST_LOG", "info") };
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Modules => cli::modules(),
        Commands::Scan { target } => cli::scan(&target),
    }
}
