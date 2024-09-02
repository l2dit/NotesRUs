use chrono::{Datelike, Local};
use jwt::SignWithKey;
use poem::web::Data;
use poem_openapi::{param::Header, payload::PlainText, payload::Json, OpenApi};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use uuid::Uuid;
use sea_orm::ActiveModelTrait;
use crate::entity::users;

use self::auth::{ServerSecret, UserToken};

use super::cli::Args;

pub mod auth;
pub mod responses;

#[derive(poem_openapi::Tags)]
pub enum ApiTags {
    /// These routes are responsible for the creation and mangment of user accounts.
    User,
    /// Route Redirects To Docs
    Redirects,
}

pub struct Api {
    pub database_connection: DatabaseConnection,
    pub args: Args,
}

/// Notes R Us API
///
/// # The Rust Documentation can be found at
/// [docs.rs/notes_r_us/latest/notes_r_us/backend](https://docs.rs/notes_r_us/latest/notes_r_us/backend)
#[OpenApi]
impl Api {
    /// Redirect The Index Path
    ///
    /// # Redirects
    /// This Redirects the user from `.../` to `.../docs`
    #[oai(path = "/", method = "get", tag = ApiTags::Redirects)]
    pub async fn index(&self) -> responses::Redirect {
        responses::Redirect::Response("/api/docs".to_string())
    }

    /// User Creation
    ///
    /// # User Creation
    /// This route is to be used to create a new user.
    ///
    /// Name param will be used on the data base side witch has not been implemted yet...
    #[oai(path = "/user/creation", method = "get", tag = ApiTags::User)]
    pub async fn create_user(
        &self,
        server_secret: Data<&ServerSecret>,
        #[oai(name = "Name")] name: Header<Option<String>>,
        #[oai(name = "ClientName")] client_name: Header<Option<String>>,
    ) -> responses::user::CreateUserResponse {
        let mut client = UserToken {
            client_secret: Uuid::new_v4()
                .sign_with_key(&server_secret.clone())
                .expect("Could Not Sign Client Secret"),
            user_name: "username".to_string(),
            ..Default::default()
        };

        match client_name.clone() {
            Some(client_name) => client.client_identifier = client_name,
            None => (),
        };

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


        let user: Result<users::Model, sea_orm::DbErr> = user.insert(&self.database_connection).await;

        match user {
            Err(error) => {
                println!("ERROR: {error:?}");
                return responses::user::CreateUserResponse::ERROR(Json(json!({"error" : format!("{error:?}"), "code":500})))
            },
            
            Ok(user) => {
                let user: users::Model = user;
                println!("{user:?}");
                return responses::user::CreateUserResponse::Ok(
                    Json(json!({
                        "message": format!("{} accont has been created", name.clone().unwrap_or("".to_string())).as_str()
                    })),
                    client.to_cookie_string(&self.args, server_secret.0.clone(), None),
                )
            }
        }


    }

    /// User Edit
    ///
    /// # Edit Name
    /// This route is to remove or change the name of the user note this is not the same as
    /// username.
    ///
    /// note this is just for testing at the moment enter a nonsence name as this is not used
    #[oai(path = "/user/edit", method = "get", tag = ApiTags::User)]
    pub async fn wow(
        &self,
        auth: auth::ApiSecurityScheme,
        #[oai(name = "NewName")] username: Header<String>,
    ) -> Json<Value> {
        Json(json!({"Info": {"ActiveUserToken": auth.0, "Name": username.clone()}}))
    }
}
