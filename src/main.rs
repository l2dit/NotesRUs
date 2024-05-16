use poem::{self, endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use poem_openapi::{
    param::{self, Path, Query},
    payload::{self, PlainText},
    OpenApi, OpenApiService,
};
use std::env::{self};

struct Api;

#[derive(poem_openapi::ApiResponse)]
enum Redirect {
    #[oai(status = 302)]
    Response(#[oai(header = "Location")] String),
}

#[OpenApi]
impl Api {
    /// Index / Docs
    #[oai(path = "/", method = "get", tag = ApiTags::API)]
    async fn index(&self) -> Redirect {
        Redirect::Response("/api/docs".to_string())
    }
}

#[derive(poem_openapi::Tags)]
enum ApiTags {
    /// All public API endpoints.
    API,
}

#[tokio::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let api_service = OpenApiService::new(
        Api,
        "Notes R Us API Documentation",
        std::env::var("CARGO_PACKAGE_VERSION").unwrap_or(String::from("N/A")),
    )
    .server("http://localhost:80")
    .server("https://testapi.nzdev.org");
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

    Server::new(TcpListener::bind("0.0.0.0:80")).run(app).await
}
