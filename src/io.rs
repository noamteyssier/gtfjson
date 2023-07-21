use flate2::read::MultiGzDecoder;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn match_output(output: &Option<String>) -> Box<dyn Write> {
    match output {
        Some(x) => Box::new(File::create(x).unwrap()),
        None => Box::new(std::io::stdout()),
    }
}

pub fn match_input(input: &str) -> Box<dyn BufRead> {
    match input.ends_with(".gz") {
        true => Box::new(BufReader::new(MultiGzDecoder::new(
            File::open(input).unwrap(),
        ))),
        false => Box::new(BufReader::new(File::open(input).unwrap())),
    }
}

