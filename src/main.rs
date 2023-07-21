mod cli;
mod convert;
mod io;
use clap::Parser;
use cli::Cli;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        cli::Command::Convert { input, output } => {
            convert::convert(&input, &output)
        }
    }
}
