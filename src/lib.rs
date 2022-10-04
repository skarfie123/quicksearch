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
