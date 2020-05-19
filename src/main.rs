use clap::App;
use log::{info, error, LevelFilter};
use crate::bingo::Bingo;

mod logger;
mod bingo;

#[macro_use]
extern crate clap;

const IMAGE_SIZE: usize = 2000;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        _ => LevelFilter::max()
    };

    logger::init(log_level).unwrap();

    let source_file = match matches.value_of("source") {
        None => {
            error!("You must specify source file");
            std::process::exit(1)
        }
        Some(s) => { s }
    };

    let cell_count: usize = matches.value_of("cells").unwrap_or("5").parse::<usize>().expect("Not a valid number");
    let output_file = matches.value_of("output").unwrap_or("bingo.png");
    let samples = matches.value_of("samples").unwrap_or("1").parse::<u32>().expect("Not a valid number");

    info!("Using source file: {}", source_file);
    info!("Grid: {0} x {0}", cell_count);

    let mut bingo = Bingo::new(IMAGE_SIZE, cell_count, source_file);

    bingo.shuffle().draw().dump_to(output_file);

    for i in 1..samples {
        bingo.shuffle().draw().dump_to(format!("{}_{}", i + 1, output_file).as_str());
    }
}
