mod cli;
mod convert;
mod io;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        cli::Command::Convert { input, output } => convert::convert(&input, &output),
    }
}
