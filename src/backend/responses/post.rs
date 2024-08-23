use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};

/// Default Value For `username` in [`PostCreationSuccess`]
fn username_default() -> String {
    "UserName".to_string()
}

/// Default Value For `post_id` in [`PostCreationSuccess`]
fn post_id_default() -> u64 {
    123u64
}

/// Post Creation Info
///
/// Gives requried info to find the post.
#[derive(Object)]
pub struct PostCreationSuccess {
    /// The Username Of the Creator
    #[oai(default = "username_default")]
    pub username: String,
    /// The Id Of The Post/Note On The Creators Account
    #[oai(default = "post_id_default")]
    pub post_id: u64,
}

/// Post Creation Response
///
/// # Responds
///
/// Tells user bacic info to find there post or how there request failed.
#[derive(ApiResponse)]
pub enum PostCreationResponse {
    /// Post Is Successfuly Created
    #[oai(status = 200)]
    PostCreated(Json<PostCreationSuccess>),
    /// Request Sender Is Not Authorised
    #[oai(status = 401)]
    Unauthorized,
    /// Bad Request With Err Message
    #[oai(status = 400)]
    Err(PlainText<String>),
}
