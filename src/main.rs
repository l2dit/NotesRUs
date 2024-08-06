use notes_r_us::backend;
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, middleware::Tracing,
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sea_orm::Database;
use std::{env, io};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use migration::{self, Migrator, MigratorTrait};

mod cli;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Parse the Args
    let args = cli::parse();

    // Database Connection
    let database = Database::connect(&args.database_url).await.unwrap();

    // Migration run
    let _ = Migrator::up(&database, None).await;

    // Set up tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    println!("{}", cli::server_url(&args, Some(String::from("/")), true));

    // Configure CORS settings
    let cors = Cors::new()
        .allow_origins(vec![
            cli::server_url(&args, None, true).as_str(),
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
    .server(cli::server_url(&args, Some(String::from("/api/")), true));
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
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        &args.origns, &args.port
    )))
    .run(app)
    .await
}
