# gtfjson

A simple CLI utility to convert a GTF file to NDJSON for fast parsing
and perform other functionalities on those jsons.

## Summary

The GTF file format is fantastic when working with `bedtools` since it is essentially
a modified version of the `BED` file format.

However, if you're interested in the annotations column, it can be a massive headache
to parse - especially if you're operating on the full genome.

I wrote this tool to convert the GTF file format into streamable newline-delim JSON.

This makes it convenient to load with `polars` in python incredibly fast and skip
all the annotation parsing.

## Installation

You can install this with the rust package manager `cargo`:

``` bash
cargo install gtfjson
```

## Usage

The executable of this tool is `gj`.

``` bash
# classic i/o
gj -i <input.gtf> -o output.json

# write to stdout
gj -i <input.gtf> 
```
