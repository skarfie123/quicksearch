use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::{collections::HashMap, env};

use crate::cli::Args;

const DEFAULT_CONFIG: &str = include_str!("default.json");

fn get_config_path() -> String {
    match env::consts::OS {
        "linux" | "macos" => format!(
            "{}/.config/quicksearch.json",
            env::var("HOME").expect("HOME environment variable not set")
        ),
        "windows" => format!(
            "{}\\quicksearch.json",
            env::var("APPDATA").expect("APPDATA environment variable not set")
        ),
        os => panic!("OS not supported: {os}"),
    }
}

pub fn parse_config() {}

#[derive(Debug, Deserialize)]
// #[allow(unused)]
pub struct QuicksearchConfig {
    /// map from keyword to url
    pub engines: HashMap<String, String>,
}

pub enum QuicksearchConfigError {
    Quicksearch,
    Config(ConfigError),
}

fn create_default_config(config_path: &str) {
    let mut file = match fs::File::create(&config_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating config file: {}", e);
            exit(1);
        }
    };
    if let Err(e) = file.write_all(DEFAULT_CONFIG.as_bytes()) {
        eprintln!("Error writing defaults to config file: {}", e);
        exit(1);
    }
}

impl QuicksearchConfig {
    pub fn parse(args: &Args) -> Result<Self, ConfigError> {
        let config_path = get_config_path();
        if !Path::new(&config_path).exists() {
            if args.verbose {
                eprintln!(
                    "{} does not exist, attempting to create config with defaults",
                    config_path
                );
            }
            create_default_config(&config_path);
            if args.verbose {
                println!("Successfully wrote default config file");
            }
        }

        let s = Config::builder()
            .add_source(File::with_name(&config_path))
            // Add in settings from the environment (with a prefix of QUICKSEARCH)
            // Eg.. `QUICKSEARCH_DEBUG=1 ./target/quicksearch` would set the `debug` key
            .add_source(Environment::with_prefix("quicksearch"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
