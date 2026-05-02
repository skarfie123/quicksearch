use std::fmt::Write;
use std::process::exit;
use std::sync::Arc;

use clap::Parser;
use quicksearch::{cli::Args, config::QuicksearchConfig, generate_url, list, search, shell};

use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};

async fn search_handler(
    Path(full_query): Path<String>,
    State(args_state): State<Arc<Args>>,
) -> impl IntoResponse {
    let full_query = full_query.replace('+', " ");
    let (keyword, query) = match full_query.split_once(' ') {
        Some((k, q)) => (k.to_string(), q),
        None => (full_query.clone(), ""),
    };
    let config = match QuicksearchConfig::parse(args_state.as_ref()) {
        Ok(config) => config,
        Err(msg) => return ([(CONTENT_TYPE, "text/plain")], msg).into_response(),
    };
    let url = if keyword == "help" {
        String::from("/")
    } else {
        match generate_url(&config.engines, &keyword, query) {
            Ok(url) => url,
            Err(_) => match config.default_engine {
                Some(keyword) => generate_url(&config.engines, &keyword, &full_query).unwrap(),
                None => {
                    return (
                        [(CONTENT_TYPE, "text/plain")],
                        String::from("Error: Engine not found"),
                    )
                        .into_response()
                }
            },
        }
    };
    Redirect::temporary(&url).into_response()
}

async fn help_handler(State(args_state): State<Arc<Args>>) -> impl IntoResponse {
    let mut html = String::from("<h1>quicksearch</h1>");
    write!(
        &mut html,
        r#"<link
    rel="search"
    type="application/opensearchdescription+xml"
    title="Quicksearch"
    href="/opensearch.xml" />"#
    )
    .unwrap();

    let config_path = quicksearch::config::get_config_path();
    write!(&mut html, "<p>Config Path: <code>{config_path}</code></p>").unwrap();
    let config = match QuicksearchConfig::parse(args_state.as_ref()) {
        Ok(config) => config,
        Err(msg) => return ([(CONTENT_TYPE, "text/plain")], msg).into_response(),
    };
    if let Some(keyword) = config.default_engine {
        write!(&mut html, "<p>Default Engine: <code>{keyword}</code></p>").unwrap();
    }
    write!(
        &mut html,
        "<table>
            <thead>
                <tr>
                    <th>Keyword</th>
                    <th>Name</th>
                    <th>Url</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>help</code></td>
                    <td>This Help Page</td>
                </tr>"
    )
    .unwrap();
    let mut keywords = config.engines.keys().collect::<Vec<_>>();
    keywords.sort();
    for keyword in keywords {
        let engine = config.engines.get(keyword).unwrap();
        let url = &engine.url;
        let name = &engine.name;
        write!(
            &mut html,
            "    <tr>
                    <td><code>{keyword}</code></td>
                    <td>{name}</td>
                    <td><code>{url}</code></td>
                </tr>"
        )
        .unwrap();
    }
    write!(
        &mut html,
        "    </tbody>
        </table>"
    )
    .unwrap();
    Html(html).into_response()
}

const OPENSEARCH_CONFIG: &str = include_str!("opensearch.xml");
async fn opensearch_handler(State(args_state): State<Arc<Args>>) -> impl IntoResponse {
    let port = match args_state.command {
        quicksearch::cli::Command::Serve(ref serve_args) => serve_args.port,
        _ => {
            return (
                [(CONTENT_TYPE, "text/plain")],
                String::from("Unexpected Error"),
            )
                .into_response()
        }
    };
    (
        [(CONTENT_TYPE, "application/opensearchdescription+xml")],
        OPENSEARCH_CONFIG.replace("[PORT]", &port.to_string()),
    )
        .into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
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
            let serve_address = format!("0.0.0.0:{}", serve_args.port);
            let shared = Arc::new(args);
            let app = Router::new()
                .route("/", get(help_handler))
                .route("/{*full_query}", get(search_handler))
                .route("/opensearch.xml", get(opensearch_handler))
                .with_state(shared);

            let listener = tokio::net::TcpListener::bind(serve_address).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        quicksearch::cli::Command::Shell(_) => shell(config, args),
        // Config command is handled earlier
        quicksearch::cli::Command::Config => panic!("Unexpected command"),
    }
    Ok(())
}
