use clap::Parser;
use notes_r_us::backend;
use poem::{self, endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::env::{self};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port For The Server
    #[arg(short, long, env, default_value_t = 3000)]
    port: u16,

    /// What Request Origns Are Valid
    #[arg(short, long, env, default_value_t = String::from("0.0.0.0"))]
    origns: String,

    /// Is The Aplication Running In Kubernetes
    #[arg(short, long, env, default_value_t = String::from("localhost"))]
    domain: String,

    /// Weather Your Server Is Being Reached From Https://
    #[arg(long, env, default_value_t = true)]
    https: bool,

    /// Postgresql Username
    #[arg(long, env)]
    postgresql_username: Option<String>,

    /// Postgresql Password
    #[arg(long, env)]
    postgresql_password: Option<String>,

    /// Postgresql Connection IP
    #[arg(long, env)]
    postgresql_ip: Option<String>,

    /// Postgresql Connection Port
    #[arg(long, env)]
    postgresql_port: Option<u16>,
}

fn server_constructor(domain: String, port: u16, https: Option<bool>) -> String {
    match https {
        Some(true) => return format!("https://{domain}:{port}"),

        Some(false) => return format!("http://{domain}:{port}"),

        None => return format!("{domain}:{port}"),
    }
}

#[tokio::main]
async fn main() -> () {
    let args = Args::parse();

    let api_service = OpenApiService::new(
        backend::Api,
        "Notes R Us API Documentation",
        env!("CARGO_PKG_VERSION"),
    )
    .server(server_constructor(args.domain, args.port, Some(args.https)));
    let ui_docs_swagger = api_service.swagger_ui();
    let app = Route::new()
        .nest(
            "/api",
            Route::new()
                .nest("/", api_service)
                .nest("/docs", ui_docs_swagger),
        )
        .nest(
            "/",
            StaticFilesEndpoint::new(env::current_dir().unwrap().join("notes_r_us_ui/dist"))
                .index_file("index.html"),
        );

    Server::new(TcpListener::bind(server_constructor(
        args.origns,
        args.port,
        None,
    )))
    .run(app)
    .await
    .unwrap();
}
