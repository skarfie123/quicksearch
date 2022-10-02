use clap::Parser;

fn main() {
    let args = quicksearch::cli::Args::parse();
    if args.verbose {
        println!("{:?}", args)
    }

    match args.command {
        quicksearch::cli::Command::List => todo!(),
        quicksearch::cli::Command::Search(_) => todo!(),
    }
}
