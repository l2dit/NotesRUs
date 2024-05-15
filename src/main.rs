use poem::{self, endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use std::env::{self};
// use poem_openapi::{self, OpenApi}

#[tokio::main]
async fn main() {
    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new(env::current_dir().unwrap().join("notes_r_us_ui/dist"))
            .index_file("index.html"),
    );

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap()
}
