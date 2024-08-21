use std::fmt::Debug;
use std::option::Option;
use chrono::Local;
use poem_openapi::{
    param::Path,
    param::Query,
    payload::{Json, PlainText},
    OpenApi,
};
use sea_orm::ActiveModelTrait;
use serde_json;
#[path = "responses/mod.rs"]
pub mod responses;
use crate::entity::users;

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

    #[oai(path = "/auth/user/register", method = "get", tag = ApiTags::API)]
    async fn create_user(&self,name: Query<Option<String>>, ) -> responses::UserRegister {
        let mut user: users::ActiveModel = users::ActiveModel {
            username: sea_orm::ActiveValue::set("zachlicious".into()),
            name: sea_orm::ActiveValue::not_set(),
            most_recent_client: sea_orm::ActiveValue::not_set(),
            role: sea_orm::ActiveValue::not_set(),
            creation_time: sea_orm::ActiveValue::set(Local::now().into()),
            ..Default::default()
        };

        if !name.is_none() {
            user.set(users::Column::Name, sea_orm::Value::from(name.clone().unwrap()))
        }
        else {
            user.set(users::Column::Name, sea_orm::Value::from(user.username.clone().unwrap()))
        };

        let user: Result<users::Model, sea_orm::DbErr> = user.insert(&self.database).await;

        match user {
            Err(user) => {
                println!("ERROR: {user:?}");
                responses::UserRegister::Error(PlainText(format!("DATABASE ERROR: {user:?}")))
            },
            Ok(user) => {
                let user: users::Model = user;
                println!("{user:?}");
                responses::UserRegister::Response(Json(serde_json::to_value(&user).unwrap()))
            }
        }
    }

    #[oai(path = "/auth/session:c_id:user_id:c_sec", method = "get", tag = ApiTags::API)]
    async fn get_token(&self, c_id: Path<String>, user_id: Path<String>, unsf_c_sec: Path<String>) -> responses::AuthSession {
        let _c_id: String = c_id.to_string();
        let _user_id: String = user_id.to_string();
        let safe_c_sec: String = unsf_c_sec.to_string(); // unsf = unhashed, safe = hashed and/or salted
        responses::AuthSession::Response(PlainText(format!("Bearer: {safe_c_sec}").to_string()), safe_c_sec.to_string())

        //TODO - Implement actual AUTH/Cookie
        // 1. Hash and (maybe) salt client secret
        // 2. Cross-Reference client data with server data (owner user id, client secret and client id)
        // 3. Return status 200 + cookie with hashed and salted client_secret on sucessful request



    }

    // #[oai(path = "/db-test/posts/new/:title", method = "post", tag = ApiTags::API)]
    // async fn upload_file(&self, upload: responses::UploadPayload, post_title: Path<String>,) -> poem::Result<Json<String>> {

    //     let user_id: i32 = 0;
    //     let time_post_upload = Local::now();
    //     let post = entity::posts::ActiveModel {
    //         title: sea_orm::ActiveValue::set(post_title.to_string()),
    //         body: sea_orm::ActiveValue::set(upload
    //             .file
    //             .into_string()
    //             .await
    //             .map_err(poem::error::BadRequest)?),
    //         user_id: sea_orm::ActiveValue::set(user_id),
    //         creation_time:  sea_orm::ActiveValue::set(time_post_upload.into()),
    //         edit_time: sea_orm::ActiveValue::set(None),
    //         ..Default::default()

    //     };
    //     let post = entity::posts::Entity::insert(post).exec(&self.database).await;

    //     println!("{post:?}");

    //     Ok(Json(pos))
    // }

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
