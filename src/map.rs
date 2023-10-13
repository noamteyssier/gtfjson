use crate::io::{match_input, match_output};
use anyhow::Result;
use clap::ValueEnum;
use gtftools::GtfReader;
use std::{collections::HashSet, io::BufWriter};

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Mappings {
    T2G,
    G2S,
    TGS,
}

pub fn mapping(input: &str, output: Option<String>, mapping: Mappings) -> Result<()> {
    // build the input handle
    let input_handle = match_input(input);

    // build the output handle
    let output_handle = match_output(&output);

    // initialize the writer
    let writer = BufWriter::new(output_handle);

    // initialize csv writer
    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer);

    let mut used_values = HashSet::new();

    for record in GtfReader::from_bufread(input_handle) {
        if let Ok(record) = record {
            match mapping {
                Mappings::T2G => {
                    // Parse the transcript id
                    let t_id = if let Some(t_id) = record.attribute.transcript_id {
                        String::from_utf8(t_id)?
                    } else {
                        continue;
                    };

                    // Skip if we've already seen this transcript id
                    if used_values.contains(&t_id) {
                        continue;
                    } else {
                        used_values.insert(t_id.clone());
                    }

                    // Parse the gene id
                    let g_id = if let Some(g_id) = record.attribute.gene_id {
                        Some(String::from_utf8(g_id)?)
                    } else {
                        None
                    };

                    if let Some(g_id) = g_id {
                        csv_writer.serialize((&t_id, &g_id))?;
                    } else {
                        csv_writer.serialize((&t_id, &t_id))?;
                    }
                }
                Mappings::G2S => {
                    // Parse the gene ID
                    let g_id = if let Some(g_id) = record.attribute.gene_id {
                        String::from_utf8(g_id)?
                    } else {
                        continue;
                    };

                    // Skip if we've already seen this gene id
                    if used_values.contains(&g_id) {
                        continue;
                    } else {
                        used_values.insert(g_id.clone());
                    }

                    // Parse the gene name
                    let g_name = if let Some(g_name) = record.attribute.gene_name {
                        Some(String::from_utf8(g_name)?)
                    } else {
                        None
                    };

                    // Write the record
                    if let Some(g_name) = g_name {
                        csv_writer.serialize((&g_id, &g_name))?;
                    } else {
                        csv_writer.serialize((&g_id, &g_id))?;
                    }
                }
                Mappings::TGS => {
                    // Parse the transcript ID
                    let t_id = if let Some(t_id) = record.attribute.transcript_id {
                        String::from_utf8(t_id)?
                    } else {
                        continue;
                    };

                    // Skip if we've already seen this transcript id
                    if used_values.contains(&t_id) {
                        continue;
                    } else {
                        used_values.insert(t_id.clone());
                    }

                    // Parse the gene ID
                    let g_id = if let Some(g_id) = record.attribute.gene_id {
                        Some(String::from_utf8(g_id)?)
                    } else {
                        None
                    };

                    // Parse the gene name
                    let g_name = if let Some(g_name) = record.attribute.gene_name {
                        Some(String::from_utf8(g_name)?)
                    } else {
                        None
                    };

                    // Write the record
                    if let Some(g_id) = g_id {
                        if let Some(g_name) = g_name {
                            csv_writer.serialize((&t_id, &g_id, &g_name))?;
                        } else {
                            csv_writer.serialize((&t_id, &g_id, &g_id))?;
                        }
                    } else {
                        if let Some(g_name) = g_name {
                            csv_writer.serialize((&t_id, &t_id, &g_name))?;
                        } else {
                            csv_writer.serialize((&t_id, &t_id, &t_id))?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
