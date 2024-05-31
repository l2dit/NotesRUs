use notes_r_us::backend;
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::Cors, middleware::Tracing,
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{env, io};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
            "http://localhost:5173"
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

    // Determine server URL based on environment
    let server_url: &str = if cfg!(debug_assertions) {
        "http://localhost:3000/api"
    } else {
        "https://notesrus.nzdev.org/api"
    };

    info!("Server URL: {}", server_url);

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
    .server(server_url.to_string());

    // Set up the application routes
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
    info!("Starting server at 0.0.0.0:3000");
    Server::new(TcpListener::bind("0.0.0.0:3000")) // Bind to all network interfaces
        .run(app)
        .await
}
