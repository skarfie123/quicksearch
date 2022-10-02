use clap::Parser;

fn main() {
    let config = quicksearch::cli::Args::parse();
    if config.verbose {
        println!("{:?}", config)
    }
}
