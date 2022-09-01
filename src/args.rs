use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
#[clap(about = "Does awesome things")]
pub struct EncodeArgs {
    #[clap(value_parser, long, short)]
    file_path: PathBuf,

    #[clap(value_parser, long, short)]
    chunk_type: String,

    #[clap(value_parser, long, short)]
    message: String,

    #[clap(value_parser, long, short)]
    output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    #[clap(value_parser, long, short)]
    file_path: PathBuf,

    #[clap(value_parser, long, short)]
    chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[clap(value_parser, long, short)]
    file_path: PathBuf,

    #[clap(value_parser, long, short)]
    chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser, long, short)]
    file_path: PathBuf,
}
