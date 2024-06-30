use clap::Parser;
use notes_r_us::{backend, cli, database};
use poem::{
    endpoint::StaticFilesEndpoint,
    listener::TcpListener,
    middleware::{Cors, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sea_orm::DatabaseConnection;
use std::{env, io};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Parse the Args
    let args = cli::Args::parse();

    let db_url: String = format!(
        "postgresql://{}:{}@{}:{}/app",
        &args.database_username.unwrap(),
        &args.database_password.unwrap(),
        &args.database_ip.unwrap(),
        &args.database_port.unwrap()
    );

    println!("{db_url}");

    let _connection: DatabaseConnection = database::setup::set_up_db(db_url.as_str(), "app")
        .await
        .expect("Could Not Connect To Data Base");

    // Set up tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    println!(
        "{}",
        cli::server_constructor(
            &args.domain,
            args.port,
            Some(String::from("/")),
            Some(args.https),
        )
    );

    // Configure CORS settings
    let cors = Cors::new()
        .allow_origins(vec![
            cli::server_constructor(&args.domain, args.port, None, Some(args.https)).as_str(),
            "http://localhost:5173",
        ])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec![
            "Authorization",
            "Content-Type",
            "X-Requested-With",
            "Accept",
            "Origin",
            "Content-Disposition",
            "*",
        ])
        .allow_credentials(true)
        .expose_headers(vec![
            "Authorization",
            "Content-Type",
            "X-Requested-With",
            "Accept",
            "Origin",
            "Content-Disposition",
            "*",
        ]);

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
    .server(cli::server_constructor(
        &args.domain,
        args.port,
        Some(String::from("/api/")),
        Some(args.https),
    ));
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
    Server::new(TcpListener::bind(cli::server_constructor(
        &args.origns,
        args.port,
        None,
        None,
    )))
    .run(app)
    .await
}
