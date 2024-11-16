// src/main.rs
mod cli;
mod commands;
mod generators;
mod templates;
mod tests;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
