use crate::cli::Commands;
use anyhow::Result;
use clap::Parser;

mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        Commands::Encode(input) => input.encode(),
        Commands::Decode(input) => input.decode(),
        Commands::Remove(input) => input.remove(),
        Commands::Print(input) => input.print(),
    }
}
