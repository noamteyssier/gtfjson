use crate::{io::match_output, types::FlatRecord};
use anyhow::Result;
use spinoff::{spinners, Color, Spinner, Streams};
use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

fn load_file(filename: &str) -> Result<BufReader<File>> {
    Ok(File::open(filename).map(BufReader::new)?)
}

pub fn partition(
    input: &str,
    variable: &str,
    output_dir: &str,
    max_open_files: usize,
) -> Result<()> {
    let mut output_dir = output_dir.to_string();
    if !output_dir.ends_with("/") {
        output_dir.push_str("/");
    }
    if !std::path::Path::new(&output_dir).exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let input_handle = load_file(input)?;

    let mut partitions = HashSet::new();
    let mut handle_map = HashMap::new();
    let mut num_records = 0;
    let mut spinner = Spinner::new_with_stream(
        spinners::Aesthetic,
        "Mapping JSON",
        Color::Green,
        Streams::Stderr,
    );
    for line in input_handle.lines() {
        let line = line?;
        let record = serde_json::from_str::<FlatRecord>(&line)?;
        if record.has_attribute(variable) {
            let value = record.attributes.get(variable).unwrap();
            let value = value.to_string().replace("\"", "");
            partitions.insert(value.clone());
            let file_path = format!("{}{}.json", output_dir, value);
            let output_handle = handle_map.entry(value.clone()).or_insert_with(|| {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .unwrap()
            });
            let json = serde_json::to_string(&record)?;
            writeln!(output_handle, "{}", json)?;

            if handle_map.len() > max_open_files {
                let drop_key = handle_map.keys().next().unwrap().clone();
                handle_map.remove(&drop_key);
            }
        }
        num_records += 1;
        if num_records % 100000 == 0 {
            spinner.update_text(format!(
                "Identified {} total partitions across {} records",
                partitions.len(),
                num_records
            ));
        }
    }
    spinner.stop_with_message(&format!(
        "Identified {} total partitions across {} records",
        partitions.len(),
        num_records
    ));
    Ok(())
}
