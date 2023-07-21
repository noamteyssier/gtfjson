use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {

    /// Convert a gtf file to ndjson
    Convert {
        #[clap(short, long)]
        input: String,
        #[clap(short, long)]
        output: Option<String>,
    }
}
