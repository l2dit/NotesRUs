use poem_openapi::{payload::Attachment, types::multipart::Upload, ApiResponse, Multipart};

#[derive(Debug, ApiResponse)]
pub enum GetFileResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
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
