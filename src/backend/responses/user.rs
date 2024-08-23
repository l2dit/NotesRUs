use poem_openapi::{payload::Json, ApiResponse};

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    /// User Is Sucessfully Created
    #[oai(status = "200")]
    Ok(
        Json<serde_json::Value>,
        #[oai(header = "Set-Cookie")] String,
    ),
    /// Error To Create User
    #[oai(status = "401")]
    Err(Json<serde_json::Value>),
}
