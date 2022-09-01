use clap::Parser;
mod args;
use args::Commands;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match &cli.command {
        Commands::Encode(args) => {
            println!("{:?}", args)
        }
        Commands::Decode(_) => todo!(),
        Commands::Remove(_) => todo!(),
        Commands::Print(_) => todo!(),
    }

    Ok(())
}
