use chrono::{TimeZone, Utc};
use chrono_tz::Pacific::Auckland;
use poem_openapi::{
    param::Path,
    payload::{Attachment, AttachmentType, Json},
    types::ToJSON,
    OpenApi,
};
use serde_json;
#[path = "responses/mod.rs"]
pub mod responses;

#[derive(poem_openapi::Tags)]
enum ApiTags {
    /// All public API endpoints.
    API,
}

#[derive(Debug, poem_openapi::Object, Clone)]
pub struct File {
    filename: Option<String>,
    data: Vec<u8>,
    upload_time: String,
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
    async fn upload_file(&self, upload: responses::UploadPayload) -> poem::Result<Json<u64>> {
        let mut status = self.status.lock().await;
        let id = status.id;
        status.id += 1;

        let utc = Utc::now().naive_utc();
        let dt = Auckland.from_utc_datetime(&utc);
        let time_file_upload: chrono::format::DelayedFormat<chrono::format::StrftimeItems> =
            dt.format("%Y-%m-%d %H:%M:%S");

        let file = File {
            filename: upload.file.file_name().map(ToString::to_string),
            data: upload
                .file
                .into_vec()
                .await
                .map_err(poem::error::BadRequest)?,
            upload_time: time_file_upload.to_string(),
        };
        status.files.insert(id, file);
        Ok(Json(id))
    }

    /// Get file
    #[oai(path = "/file/download/:id", method = "get", tag = ApiTags::API)]
    async fn get_file(&self, id: Path<u64>) -> responses::GetFileResponse {
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

    /// Delete markdown file
    #[oai(path = "/file/delete/:id", method = "delete", tag = ApiTags::API)]
    async fn delete_file(&self, id: Path<u64>) -> responses::DeleteFileResponse {
        let mut status = self.status.lock().await;
        if status.files.get(&id).is_some() {
            status.files.remove(&id).unwrap();

            responses::DeleteFileResponse::Ok(Json(format!(
                "Deleted file with id: {}",
                id.to_string()
            )))
        } else {
            responses::DeleteFileResponse::NotFound
        }
    }

    /// View file markdown content
    #[oai(path = "/file/view/:id", method = "get", tag = ApiTags::API)]
    async fn view_file(&self, id: Path<u64>) -> responses::ViewFileResponse {
        let status = self.status.lock().await;
        match status.files.get(&id) {
            Some(file) => {
                let file_content: String = String::from_utf8(file.data.clone()).unwrap();

                responses::ViewFileResponse::Ok(poem_openapi::payload::PlainText(file_content))
            }
            None => responses::ViewFileResponse::NotFound,
        }
    }

    /// Get all files
    #[oai(path = "/file/all", method = "get", tag = ApiTags::API)]
    async fn get_all_files(&self) -> poem::Result<Json<serde_json::Value>> {
        let status = self.status.lock().await;

        Ok(Json(status.files.to_json().unwrap()))
    }
}
