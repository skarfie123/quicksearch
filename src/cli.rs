use clap::{Parser, Subcommand, ValueEnum};

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
    /// Print the config path
    Config,
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
    /// Provide the commands for shell integration
    ///
    /// This needs to be used again every time the config is updated.
    ///
    /// Eg. for zsh, put `eval "$(quicksearch shell zsh)"` in your ~/.zshrc so that it is
    /// updated every time you restart your shell.
    Shell(ShellArgs),
}

#[derive(Parser, Debug)]
pub struct ShellArgs {
    // #[arg(value_enum)]
    pub shell_type: ShellType,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ShellType {
    BASH,
    FISH,
    PWSH,
    ZSH,
}

#[derive(Parser, Debug)]
pub struct SearchArgs {
    /// keyword corresponding to a search engine
    pub keyword: String,
    /// query for the search engine
    pub query: Vec<String>,
}
