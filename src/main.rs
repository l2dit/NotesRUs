use notes_r_us::backend;
use poem::{self, endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::env::{self};

#[tokio::main]
async fn main() -> () {
    let server_url: &str = if cfg!(debug_assertions) {
        "localhost:3000"
    } else {
        "notesrus.nzdev.org"
    };

    let api_service = OpenApiService::new(
        backend::Api,
        "Notes R Us API Documentation",
        env!("CARGO_PKG_VERSION"),
    )
    .server(format!("http://{}", server_url));
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

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .unwrap();
}
