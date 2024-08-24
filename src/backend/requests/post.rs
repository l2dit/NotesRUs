use poem_openapi::{payload::Json, ApiRequest, Object};
use serde::Serialize;

/// Default Value For `title` In [`PostCreation`]
fn title() -> String {
    "Title".to_string()
}

/// Default Value For `body` In [`PostCreation`]
fn body() -> String {
    "BodyContent".to_string()
}

/// Body of the Post/Note Creation
#[derive(Object, Serialize)]
pub struct PostBody {
    /// The Title Of You Post/Note
    #[oai(default = "title")]
    title: String,
    /// The Main Body/Content Of Your Post/Note
    #[oai(default = "body")]
    body: String,
}

/// Post/Note Creation
#[derive(ApiRequest)]
pub enum PostCreation {
    /// Json Request Body
    CreatePost(Json<PostBody>),
}

/// Post/Note Eddition
#[derive(ApiRequest)]
pub enum PostEdition {
    /// Json Request Body
    EditPost(Json<PostBody>),
}
