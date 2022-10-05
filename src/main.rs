use std::process::exit;

use clap::Parser;
use quicksearch::{list, search};
fn main() {
    let args = quicksearch::cli::Args::parse();
    if args.verbose {
        println!("CLI Args: {:?}", args)
    }
    // Handle this early as we shouldn't attempt to parse the config if the user just wants the path
    if let quicksearch::cli::Command::Config = args.command {
        println!("{}", quicksearch::config::get_config_path());
        exit(0)
    }
    let config = match quicksearch::config::QuicksearchConfig::parse(&args) {
        Ok(config) => config,
        Err(e) => {
            match e {
                config::ConfigError::FileParse { uri: _, cause } => {
                    eprintln!("Error while parsing config: {cause}")
                }
                config::ConfigError::Message(msg) => eprintln!("Error while parsing config: {msg}"), // at least Missing fields
                e => eprintln!("Unexpected error while parsing config: {e}"),
            }
            exit(1)
        }
    };

    match args.command {
        quicksearch::cli::Command::List => list(config),
        quicksearch::cli::Command::Search(_) => search(config, args),
        // Config command is handled earlier
        quicksearch::cli::Command::Config => panic!("Unexpected command"),
    }
}
