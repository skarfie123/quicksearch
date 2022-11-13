use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::{collections::HashMap, env};

use crate::cli::Args;

const DEFAULT_CONFIG: &str = include_str!("default.json");

pub fn get_config_path() -> String {
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
pub struct QuicksearchConfig {
    /// Map from keyword to engine config
    pub engines: HashMap<String, EngineConfig>,
    /// engine to fall back to when the first work doesn't match an engine keyword
    pub default_engine: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    /// name of engine
    pub name: String,
    /// url with %s to indicate search terms substitution
    pub url: String,
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
    fn _parse(args: &Args) -> Result<Self, ConfigError> {
        let config_path = get_config_path();
        if !Path::new(&config_path).exists() {
            println!(
                "{} does not exist, creating config with defaults",
                config_path
            );
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

    pub fn parse(args: &Args) -> Result<Self, String> {
        match Self::_parse(args) {
            Ok(config) => Self::validate(config),
            Err(e) => {
                match e {
                    config::ConfigError::FileParse { uri: _, cause } => {
                        Err(format!("Error while parsing config: {cause}"))
                    }
                    config::ConfigError::Message(msg) => {
                        Err(format!("Error while parsing config: {msg}"))
                    } // at least Missing fields
                    e => Err(format!("Unexpected error while parsing config: {e}")),
                }
            }
        }
    }

    fn validate(config: Self) -> Result<Self, String> {
        if let Some(ref keyword) = config.default_engine {
            if !config.engines.contains_key(keyword) {
                return Err(format!(
                    "'{keyword}' set as default engine, but engine does not exist"
                ));
            }
        }

        if config.engines.contains_key("help") {
            return Err(
                "'help' set as engine keyword, but it is reserved for the help page in server mode"
                    .to_string(),
            );
        }

        Ok(config)
    }
}
