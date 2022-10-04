use clap::{Parser, Subcommand};

/// Quicksearch
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    /// verbose logging mode
    #[clap(short, long, global(true))]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List available search engines
    ///
    /// engines will be listed as `keyword: url`
    List,
    /// Perform a search
    ///
    /// %s in the url will be replaced with your query
    ///
    /// For convenience you can set an alias in your shell.
    ///
    /// eg. for zsh: `alias q="quicksearch search"`
    Search(SearchArgs),
}

#[derive(Parser, Debug)]
pub struct SearchArgs {
    /// keyword corresponding to a search engine
    pub keyword: String,
    /// query for the search engine
    pub query: Vec<String>,
}
