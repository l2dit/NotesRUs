use poem_openapi::{payload::Json, ApiRequest, Object};

/// Default Value For `title` In [`PostCreationBody`]
fn title_default() -> String {
    "Title".to_string()
}

/// Default Value For `body` In [`PostCreationBody`]
fn body_default() -> String {
    "BodyContent".to_string()
}

/// Default Value For `post_id` in [`PostDeletionBody`]
fn post_id_default() -> u64 {
    123u64
}

/// Body of the Post/Note Creation Request
#[derive(Object)]
pub struct PostBody {
    /// The Title Of You Post/Note
    #[oai(default = "title_default")]
    title: String,
    /// The Main Body/Content Of Your Post/Note
    #[oai(default = "body_default")]
    body: String,
}

/// Body Of Post/Note Deletion Request
#[derive(Object)]
pub struct PostDeletionBody {
    /// Post/Note You Want To Be Deleted
    #[oai(default = "post_id_default")]
    post_id: u64,
}

/// Post/Note Creation
#[derive(ApiRequest)]
pub enum PostCreation {
    /// Json Request Body
    CreatePost(Json<PostBody>),
}

/// Post/Note Edition
#[derive(ApiRequest)]
pub enum PostEdition {
    /// Json Request Body
    EditPost(Json<PostBody>),
}

/// Post/Note Deletion
#[derive(ApiRequest)]
pub enum PostDeletion {
    /// Requests Json Body
    DeletePost(Json<PostDeletionBody>),
}
