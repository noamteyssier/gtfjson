use std::{fs::File, io::{BufReader, BufRead, BufWriter, Write}, collections::HashMap};
use anyhow::Result;
use crate::{types::FlatRecord, io::match_output};

fn load_file(filename: &str) -> Result<BufReader<File>> {
    Ok(File::open(filename).map(BufReader::new)?)
}

pub fn partition(
    input: &str,
    variable: &str,
    output_dir: &str,
) -> Result<()> {

    let mut output_dir = output_dir.to_string();
    if !output_dir.ends_with("/") {
        output_dir.push_str("/");
    }
    if !std::path::Path::new(&output_dir).exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let handle = load_file(input)?;
    let mut map = HashMap::new();
    for line in handle.lines() {
        let line = line?;
        let record = serde_json::from_str::<FlatRecord>(&line)?;
        if record.has_attribute(variable) {
            let value = record.attributes.get(variable).unwrap();
            let value = value.to_string().replace("\"", "");
            map.entry(value).or_insert(vec![]).push(record);
        }
    }

    for key in map.keys() {
        let output_handle = match_output(&Some(format!("{}{}.json", output_dir, key)));
        let mut writer = BufWriter::new(output_handle);
        let records = map.get(key).unwrap();
        for record in records {
            let json = serde_json::to_string(&record)?;
            writeln!(writer, "{}", json)?;
        }
    }

    Ok(())
}
