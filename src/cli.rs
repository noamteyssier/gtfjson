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
        /// Input GTF file to convert
        #[clap(short, long)]
        input: String,

        /// Output file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Partition gtf-ndjson file by variable
    /// and write to separate ndjson files
    Partition {
        /// Input ndjson gtf file to partition (default=stdin)
        #[clap(short, long)]
        input: String,

        /// Variable to partition by
        #[clap(short, long, default_value = "gene_name")]
        variable: String,

        /// Output directory to write to
        #[clap(short, long, default_value = ".")]
        output: String,
    },
}
