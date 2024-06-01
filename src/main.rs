use clap::Parser;
use notes_r_us::backend;
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, middleware::Tracing,
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{env, io};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
async fn main() -> io::Result<()> {
    // Set up tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Configure CORS settings
    let cors = Cors::new()
        .allow_origins(vec![
            "http://localhost:5173",
            "http://localhost:3000",
            "https://notesrus.nzdev.org",
        ])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec![
            "Authorization",
            "Content-Type",
            "X-Requested-With",
            "Accept",
            "Origin",
        ])
        .allow_credentials(true)
        .expose_headers(vec![
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Headers",
        ]);

    let args = Args::parse();

    // Create the API service
    let api_service = OpenApiService::new(
        backend::Api {
            status: tokio::sync::Mutex::new(backend::Status {
                id: 1,
                files: Default::default(),
            }),
        },
        "Notes R Us API Documentation",
        env!("CARGO_PKG_VERSION"),
    )
    // Set up the application routes
    .server("http://localhost:3000/api");
    let ui_docs_swagger = api_service.swagger_ui();

    // Apply CORS middleware to the routes
    let app = Route::new()
        .nest(
            "/api",
            Route::new()
                .nest("/", api_service)
                .nest("/docs", ui_docs_swagger)
                .with(cors)
                .with(Tracing),
        )
        .nest(
            "/",
            StaticFilesEndpoint::new(env::current_dir().unwrap().join("notes_r_us_ui/dist"))
                .index_file("index.html"),
        );
    // Start the server
    Server::new(TcpListener::bind(server_constructor(
        args.origns,
        args.port,
        None,
    )))
    .run(app)
    .await
}
