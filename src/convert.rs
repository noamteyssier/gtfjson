use std::io::{BufWriter, Write};
use anyhow::Result;
use gtftools::GtfReader;
use crate::io::{match_input, match_output};


pub fn convert(
    input_path: &str,
    output_path: &Option<String>,
) -> Result<()> {
    // build the input handle
    let input_handle = match_input(input_path);

    // build the output handle
    let output_handle = match_output(output_path);

    // initialize the writer
    let mut writer = BufWriter::new(output_handle);

    // read in gtf records and parse to json
    let json_records = GtfReader::from_bufread(input_handle)
        .filter_map(|x| x.ok())
        .map(|x| serde_json::to_string(&x).unwrap());

    // write newline delimited json to output handle
    for j in json_records {
        writeln!(writer, "{j}")?;
    }

    Ok(())
}
