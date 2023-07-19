use anyhow::Result;
use std::{fs::File, io::{BufReader, BufWriter, Write, BufRead}};
use flate2::read::MultiGzDecoder;
use gtftools::GtfReader;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: Option<String>,
}

fn match_output(output: Option<String>) -> Box<dyn Write> {
    match output {
        Some(x) => Box::new(File::create(x).unwrap()),
        None => Box::new(std::io::stdout()),
    }
}

fn match_input(input: String) -> Box<dyn BufRead> {
    match input.ends_with(".gz") {
        true => Box::new(BufReader::new(MultiGzDecoder::new(File::open(input).unwrap()))),
        false => Box::new(BufReader::new(File::open(input).unwrap())),
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // build the input handle
    let input_handle = match_input(args.input);

    // build the output handle
    let output_handle = match_output(args.output);

    // initialize the writer
    let mut writer = BufWriter::new(output_handle);

    // read in gtf records and parse to json
    let json_records = GtfReader::from_bufread(input_handle)
      .filter_map(|x| x.ok())
      .map(|x| {
          serde_json::to_string(&x).unwrap()
      });

    // write newline delimited json to output handle
    for j in json_records {
        writeln!(writer, "{j}")?;
    }

    Ok(())
}

