use clap::Parser;

/// Quicksearch CLI Args
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// keyword corresponding to a search engine
    pub keyword: String,
    /// query for the search engine
    pub query: Vec<String>,
    /// verbose logging mode
    #[clap(short, long)]
    pub verbose: bool,
}
