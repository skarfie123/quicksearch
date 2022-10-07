use std::process::exit;

use clap::Parser;
use quicksearch::{cli::Args, config::QuicksearchConfig, generate_url, list, search, shell};

#[macro_use]
extern crate rocket;

use rocket::{config::Config, http::ContentType, State};

#[get("/<full_query>")]
fn search_handler(full_query: &str, args_state: &State<Args>) -> (ContentType, String) {
    let (keyword, query) = match full_query.split_once(" ") {
        Some(result) => result,
        None => (full_query, ""),
    };
    let config = match QuicksearchConfig::parse(args_state.inner()) {
        Ok(config) => config,
        Err(msg) => return (ContentType::Plain, msg),
    };
    let url = match generate_url(&config.engines, keyword, query) {
        Ok(url) => url,
        Err(_) => return (ContentType::Plain, "Error: Engine not found".into()),
    };
    (
        ContentType::HTML,
        format!("<meta http-equiv=\"Refresh\" content=\"0; url='{url}'\" />"),
    )
}

#[get("/")]
fn help_handler(args_state: &State<Args>) -> (ContentType, String) {
    let mut html = String::from("<h1>quicksearch</h1>");
    let config_path = quicksearch::config::get_config_path();
    html += &format!("<p>Config path: <code>{config_path}</code></p>");
    let config = match QuicksearchConfig::parse(args_state.inner()) {
        Ok(config) => config,
        Err(msg) => return (ContentType::Plain, msg),
    };
    let mut keywords = config.engines.keys().collect::<Vec<_>>();
    keywords.sort();
    for keyword in keywords {
        let url = config.engines.get(keyword).unwrap();
        html += &format!("<p><code>{keyword} - {url}</code></p>")
    }
    (ContentType::HTML, html)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args = Args::parse();
    if args.verbose {
        println!("CLI Args: {:?}", args)
    }
    // Handle this early as we shouldn't attempt to parse the config if the user just wants the path
    if let quicksearch::cli::Command::Config = args.command {
        println!("{}", quicksearch::config::get_config_path());
        exit(0)
    }
    let config = match QuicksearchConfig::parse(&args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    };

    match args.command {
        quicksearch::cli::Command::List => list(config),
        quicksearch::cli::Command::Search(_) => search(config, args),
        quicksearch::cli::Command::Serve(ref serve_args) => {
            let config = Config::figment().merge(("port", serve_args.port));
            let _rocket = rocket::custom(config)
                .mount("/", routes![search_handler, help_handler])
                .manage(args)
                .launch()
                .await?;
        }
        quicksearch::cli::Command::Shell(_) => shell(config, args),
        // Config command is handled earlier
        quicksearch::cli::Command::Config => panic!("Unexpected command"),
    }
    Ok(())
}
