use poem_openapi::{
    payload::{Attachment, Json, PlainText},
    types::multipart::Upload,
    ApiResponse, Multipart,
};
use serde_json;


#[derive(Debug, ApiResponse)]
pub enum GetFileResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
    /// File not found
    #[oai(status = 404)]
    NotFound,
}

#[derive(Debug, ApiResponse)]
pub enum ViewFileResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    /// File not found
    #[oai(status = 404)]
    NotFound,
}

#[derive(Debug, ApiResponse)]
pub enum DeleteFileResponse {
    #[oai(status = 200)]
    Ok(Json<String>),
    /// File not found
    #[oai(status = 404)]
    NotFound,
}

#[derive(Debug, Multipart)]
pub struct UploadPayload {
    pub file: Upload,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Redirect {
    #[oai(status = 302)]
    Response(#[oai(header = "Location")] String),
}
#[derive(poem_openapi::ApiResponse)]

pub enum UserRegister {
    #[oai(status = 200)]
    Response(Json<serde_json::Value>),

    #[oai(status = 500)]
    Error(PlainText<String>)

}

#[derive(poem_openapi::ApiResponse)]
pub enum AuthSession {
    #[oai(status = 200)]
    Response(PlainText<String>, #[oai(header = "Set-Cookie")] String),
}

