use poem_openapi::{
    param::Path,
    payload::{Attachment, AttachmentType, Json},
    OpenApi,
};
#[path = "responses/mod.rs"]
pub mod responses;

#[derive(poem_openapi::Tags)]
enum ApiTags {
    /// All public API endpoints.
    API,
}

#[derive(Debug, poem_openapi::Object, Clone)]
pub struct File {
    content_type: Option<String>,
    filename: Option<String>,
    data: Vec<u8>,
}

pub struct Status {
    pub id: u64,
    pub files: std::collections::HashMap<u64, File>,
}

pub struct Api {
    pub status: tokio::sync::Mutex<Status>,
}

#[OpenApi]
impl Api {
    /// Index / Docs
    #[oai(path = "/", method = "get", tag = ApiTags::API)]
    async fn index(&self) -> responses::Redirect {
        responses::Redirect::Response("/api/docs".to_string())
    }

    /// Upload file
    #[oai(path = "/file/upload", method = "post", tag = ApiTags::API)]
    async fn upload(&self, upload: responses::UploadPayload) -> poem::Result<Json<u64>> {
        let mut status = self.status.lock().await;
        let id = status.id;
        status.id += 1;

        let file = File {
            content_type: upload.file.content_type().map(ToString::to_string),
            filename: upload.file.file_name().map(ToString::to_string),
            data: upload
                .file
                .into_vec()
                .await
                .map_err(poem::error::BadRequest)?,
        };
        status.files.insert(id, file);
        Ok(Json(id))
    }

    /// Get file
    #[oai(path = "/file/download/:id", method = "get", tag = ApiTags::API)]
    async fn get(&self, id: Path<u64>) -> responses::GetFileResponse {
        let status = self.status.lock().await;
        match status.files.get(&id) {
            Some(file) => {
                let mut attachment =
                    Attachment::new(file.data.clone()).attachment_type(AttachmentType::Attachment);
                if let Some(filename) = &file.filename {
                    attachment = attachment.filename(filename);
                }
                responses::GetFileResponse::Ok(attachment)
            }
            None => responses::GetFileResponse::NotFound,
        }
    }
}
