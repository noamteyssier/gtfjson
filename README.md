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

### Convert

To convert GTF file formats to NDJSON we can use the `convert` subcommand

``` bash
# classic i/o
gj convert -i <input.gtf> -o output.json

# write to stdout
gj convert -i <input.gtf> 
```

### Partition

We can also use `gj` to partition a gtf-json in different ways.

It takes a variable in the attributes and creates a new file for each
category of that record and populates those files with the records matching
that category.

For example - we can write the GTF of every gene to a separate file:

``` bash
# Partition on gene_name
gj partition -i <input.ndjson> -o partitions/ -v gene_name

# Partition of gene_id
gj partition -i <input.ndjson> -o partitions/ -v gene_id

# Partition of transcript_biotype
gj partition -i <input.ndjson> -o partitions/ -v transcript_biotype
```
