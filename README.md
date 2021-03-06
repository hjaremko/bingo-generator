# Bingo Generator ![Rust](https://github.com/hjaremko/bingo-generator/workflows/Rust/badge.svg?branch=master)

## Running
```
cargo run --release -- <command line options>
```

## Usage
```
USAGE:
    ./bingo_generator [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Print log messages

OPTIONS:
    -c, --cells <COUNT>      Specify grid size (default 5)
    -o, --output <FILE>      Specify output file (default bingo.png)
    -i, --samples <VALUE>    Specify amount of output files produced
    -s, --source <FILE>      Specify source file
```
