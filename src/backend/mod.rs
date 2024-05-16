use poem_openapi::{
    param::{self, Path, Query},
    payload::{self, PlainText},
    OpenApi,
};

pub struct Api;

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
