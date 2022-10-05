use std::process::exit;

use urlencoding::encode;

use crate::config::QuicksearchConfig;

pub mod cli;
pub mod config;

pub fn list(config: QuicksearchConfig) {
    let mut keywords = config.engines.keys().collect::<Vec<_>>();
    keywords.sort();
    for keyword in keywords {
        println!("{keyword}: {}", config.engines.get(keyword).unwrap());
    }
}

pub fn search(config: QuicksearchConfig, args: cli::Args) {
    let search_args = match args.command {
        cli::Command::Search(args) => args,
        _ => panic!("expected search command"),
    };
    let url = match generate_url(&config, &search_args) {
        Ok(url) => url,
        Err(_) => {
            eprintln!(
                "Error: engine '{}' not found in config",
                search_args.keyword
            );
            exit(1)
        }
    };
    if args.verbose {
        println!("search url: {url}");
    }
    match open::that(url) {
        Ok(_) if args.verbose => println!("Url open succeeded"),
        Err(e) => eprint!("Error opening url: {e}"),
        _ => (),
    };
}

pub fn generate_url(config: &QuicksearchConfig, args: &cli::SearchArgs) -> Result<String, ()> {
    let url = config.engines.get(&args.keyword);
    match url {
        Some(url) => Ok(url.replace("%s", &encode(&args.query.join(" ")))),
        None => Err(()),
    }
}
