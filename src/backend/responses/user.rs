use poem_openapi::{payload::Json, ApiResponse};

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    /// User Is Sucessfully Created
    #[oai(status = "200")]
    Ok(
        Json<serde_json::Value>,
        #[oai(header = "Set-Cookie")] String,
    ),
    /// User Failed To Be Created
    #[oai(status = 500)]
    ERROR(Json<serde_json::Value>)
}
