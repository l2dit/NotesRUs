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

/// Post/Note Creation Info
///
/// Gives requried info to find the post.
#[derive(Object)]
pub struct PostResponseSuccess {
    /// The Username Of the Creator
    #[oai(default = "username_default")]
    pub username: String,
    /// The Id Of The Post/Note On The Creators Account
    #[oai(default = "post_id_default")]
    pub post_id: u64,
}

/// Post/Note Creation Response
///
/// # Responds
///
/// Tells user bacic info to find there post or how there request failed.
#[derive(ApiResponse)]
pub enum PostCreationResponse {
    /// Post Is Successfuly Created
    #[oai(status = 201)]
    PostCreated(Json<PostResponseSuccess>),
    /// Bad Request With Error Response
    #[oai(status = 400)]
    Err(PlainText<String>),
}

/// Post/Note Edition Response
///
/// # Responds
///
/// Tells the user basic info to find there post or show why there request failed.
#[derive(ApiResponse)]
pub enum PostEditionResponse {
    /// Edit Completed Successfuly
    #[oai(status = 200)]
    PostEdtion(Json<PostResponseSuccess>),
    // Requesting User Is Unauthorised To Preform Action
    #[oai(status = 403)]
    Forbiden,
    /// Bad Request With Error Response
    ///
    /// Dev notes: Might Not be needed...
    #[oai(status = 400)]
    Err(PlainText<String>),
}

/// Post/Note Deletion Response
///
/// # Responds
///
/// Lets the user know if there action was successfull or if it was forbiden
#[derive(ApiResponse)]
pub enum PostDeletionResponse {
    /// Deletion Was Completed Successfuly
    #[oai(status = 200)]
    PostDeletion(Json<PostResponseSuccess>),
    /// Requesting User Is Unautorized To Preform Action
    #[oai(status = 403)]
    Forbiden,
}
