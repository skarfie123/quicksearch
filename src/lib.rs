use std::{collections::HashMap, process::exit};

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

pub fn shell(config: QuicksearchConfig, args: cli::Args) {
    let shell_args = match args.command {
        cli::Command::Shell(args) => args,
        _ => panic!("expected shell command"),
    };
    for keyword in config.engines.keys() {
        match shell_args.shell_type {
            cli::ShellType::BASH | cli::ShellType::FISH | cli::ShellType::ZSH => {
                println!("alias {keyword}=\"quicksearch search {keyword}\"")
            }
            cli::ShellType::PWSH => {
                println!("function {keyword} {{ quicksearch search {keyword} $args }}")
            }
        }
    }
}

pub fn search(config: QuicksearchConfig, args: cli::Args) {
    let search_args = match args.command {
        cli::Command::Search(args) => args,
        _ => panic!("expected search command"),
    };
    let url = match generate_url(
        &config.engines,
        &search_args.keyword,
        &search_args.query.join(" "),
    ) {
        Ok(url) => url,
        Err(EngineNotFound) => {
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

#[derive(Debug)]
pub struct EngineNotFound;

pub fn generate_url(
    engines: &HashMap<String, String>,
    keyword: &str,
    query: &str,
) -> Result<String, EngineNotFound> {
    let url = engines.get(keyword);
    match url {
        Some(url) => Ok(url.replace("%s", &encode(query))),
        None => Err(EngineNotFound),
    }
}
