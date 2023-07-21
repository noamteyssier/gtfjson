mod cli;
mod convert;
mod io;
mod partition;
mod types;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        cli::Command::Convert { input, output } => convert::convert(&input, &output),
        cli::Command::Partition {
            input,
            variable,
            output,
            max_open_files,
        } => partition::partition(&input, &variable, &output, max_open_files),
    }
}
