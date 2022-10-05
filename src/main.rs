use std::process::exit;

use clap::Parser;
use quicksearch::{list, search};
fn main() {
    let args = quicksearch::cli::Args::parse();
    if args.verbose {
        println!("CLI Args: {:?}", args)
    }
    let config = match quicksearch::config::QuicksearchConfig::parse(&args) {
        Ok(config) => config,
        Err(e) => {
            match e {
                config::ConfigError::FileParse { uri: _, cause } => {
                    eprintln!("Error parsing config file: {cause}")
                }

                config::ConfigError::Message(msg) => eprintln!("Error: {msg}"), // at least Missing fields
                e => eprintln!("Unexpected error: {e}"),
            }
            exit(1)
        }
    };

    match args.command {
        quicksearch::cli::Command::List => list(config),
        quicksearch::cli::Command::Search(_) => search(config, args),
        quicksearch::cli::Command::Config => println!("{}", quicksearch::config::get_config_path()),
    }
}
