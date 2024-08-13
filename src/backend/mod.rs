use std::fmt::Debug;

use chrono::{TimeZone, Utc};
use chrono_tz::Pacific::Auckland;
use poem_openapi::{
    param::Path,
    payload::{Attachment, AttachmentType, Json, PlainText},
    types::ToJSON,
    OpenApi,
};
use poem::Request;
use sea_orm::EntityTrait;
use uuid::Uuid;
use serde_json;
#[path = "responses/mod.rs"]
pub mod responses;
use crate::cli;

use super::{entity::prelude, entity::posts};

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

pub struct Api {
    pub database: sea_orm::DatabaseConnection,
}

#[OpenApi]
impl Api {
    /// Index / Docs
    #[oai(path = "/", method = "get", tag = ApiTags::API)]
    async fn index(&self) -> responses::Redirect {
        responses::Redirect::Response("/api/docs".to_string())
    }

    #[oai(path = "/auth/session:c_id:user_id:c_sec", method = "get", tag = ApiTags::API)]
    async fn get_cookie(&self, c_id: Path<String>, user_id: Path<String>, unsf_c_sec: Path<String>) -> responses::AuthSession {
        let c_id: String = c_id.to_string();
        let user_id: String = user_id.to_string();
        let safe_c_sec: String = unsf_c_sec.to_string(); // unsf = unhashed, safe = hashed and/or salted
        responses::AuthSession::Response(PlainText(format!("Client/Device ID: {c_id}\nUsr ID: {user_id}\nHashed Secret: {safe_c_sec}").to_string()), safe_c_sec.to_string())
    }

    /// Upload file
    #[oai(path = "/db-test/posts/new/:title", method = "post", tag = ApiTags::API)]
    async fn upload_file(&self, upload: responses::UploadPayload, post_title: Path<String>) -> poem::Result<Json<String>> {

        let id = uuid::Uuid::new_v4();
        let time_post_upload =
    Auckland.from_utc_datetime(&Utc::now().naive_utc());
        let post = posts::ActiveModel {
            id: sea_orm::ActiveValue::set(id.clone().to_string()),
            title: sea_orm::ActiveValue::set(post_title.to_string()),
            body: sea_orm::ActiveValue::set(upload
                .file
                .into_string()
                .await
                .map_err(poem::error::BadRequest)?),
            user_id: sea_orm::ActiveValue::set(id.clone().to_string()),
            creation_time:  sea_orm::ActiveValue::set(chrono::DateTime::parse_from_str(time_post_upload.format("%Y-%m-%d %H:%M:%S").to_string().as_str(), time_post_upload.offset().to_string().as_str()).unwrap()),
            up_votes: sea_orm::ActiveValue::set(Some(0)),
            down_votes: sea_orm::ActiveValue::set(Some(0)),
            edit_time: sea_orm::ActiveValue::set(Some(chrono::DateTime::parse_from_str(time_post_upload.format("%Y-%m-%d %H:%M:%S").to_string().as_str(), time_post_upload.offset().to_string().as_str()).unwrap()))

        };
        let post = posts::Entity::insert(post).exec(&self.database).await;

        println!("{post:?}");

        Ok(Json(id.to_string()))
    }

    // /// Get file
    // #[oai(path = "/file/download/:id", method = "get", tag = ApiTags::API)]
    // async fn get_file(&self, id: Path<u64>) -> responses::GetFileResponse {
    //     let status = self.status.lock().await;
    //     match status.files.get(&id) {
    //         Some(file) => {
    //             let mut attachment =
    //                 Attachment::new(file.data.clone()).attachment_type(AttachmentType::Attachment);
    //             if let Some(filename) = &file.filename {
    //                 attachment = attachment.filename(filename);
    //             }
    //             responses::GetFileResponse::Ok(attachment)
    //         }
    //         None => responses::GetFileResponse::NotFound,
    //     }
    // }

    // /// Delete markdown file
    // #[oai(path = "/file/delete/:id", method = "delete", tag = ApiTags::API)]
    // async fn delete_file(&self, id: Path<u64>) -> responses::DeleteFileResponse {
    //     let mut status = self.status.lock().await;
    //     if status.files.get(&id).is_some() {
    //         status.files.remove(&id).unwrap();

    //         responses::DeleteFileResponse::Ok(Json(format!(
    //             "Deleted file with id: {}",
    //             id.to_string()
    //         )))
    //     } else {
    //         responses::DeleteFileResponse::NotFound
    //     }
    // }

    // /// View file markdown content
    // #[oai(path = "/file/view/:id", method = "get", tag = ApiTags::API)]
    // async fn view_file(&self, id: Path<u64>) -> responses::ViewFileResponse {
    //     let status = self.status.lock().await;
    //     match status.files.get(&id) {
    //         Some(file) => {
    //             let file_content: String = String::from_utf8(file.data.clone()).unwrap();

    //             responses::ViewFileResponse::Ok(poem_openapi::payload::PlainText(file_content))
    //         }
    //         None => responses::ViewFileResponse::NotFound,
    //     }
    // }

    // /// Get all files
    // #[oai(path = "/file/all", method = "get", tag = ApiTags::API)]
    // async fn get_all_files(&self) -> poem::Result<Json<serde_json::Value>> {
    //     let status = self.status.lock().await;

    //     Ok(Json(status.files.to_json().unwrap()))
    // }

}
