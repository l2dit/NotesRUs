use crate::{
    backend::{
        auth::{ServerSecret, UserToken},
        requests::post::{PostCreation, PostEdition, PostSelection},
        responses::post::{
            PostCreationResponse, PostDeletionResponse, PostEditionResponse, PostGetResponse,
            PostResponseSuccess,
        },
    },
    entity::users,
};
use chrono::Local;
use jwt::SignWithKey;
use names::{Generator, Name};
use poem::web::Data;
use poem_openapi::{
    param::{Header, Query},
    payload::Json,
    OpenApi, Tags,
};
use sea_orm::EntityTrait;
use sea_orm::Set as DataBaseSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter};
use sea_orm::{DatabaseConnection, IntoActiveModel};
use serde_json::json;
use uuid::Uuid;

use super::cli::Args;

pub mod auth;
pub mod requests;
pub mod responses;

#[derive(Tags)]
pub enum ApiTags {
    /// These routes are responsible for the creation and mangment of user accounts.
    User,
    /// Route Redirects To Docs
    Redirects,
    /// Post Managemet
    Post,
}

pub struct Api {
    pub database_connection: DatabaseConnection,
    pub args: Args,
}

// for development pruposes only should be removed
#[allow(unused_variables)]

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
        // initlises the client object minuns the username
        let mut client = UserToken {
            client_secret: Uuid::new_v4()
                .sign_with_key(&server_secret.clone())
                .expect("Could Not Sign Client Secret"),
            client_identifier: match client_name.0 {
                Some(client_name) => client_name,
                None => Generator::with_naming(Name::Plain)
                    .next()
                    .unwrap()
                    .to_string(),
            },
            ..Default::default()
        };

        // user with out the username that includes there id `{username}` not `{username}-{id}`
        let user: users::ActiveModel = users::ActiveModel {
            username: sea_orm::ActiveValue::set(
                Generator::with_naming(Name::Plain).next().unwrap(),
            ),
            name: match name.clone() {
                Some(name) => sea_orm::ActiveValue::set(name.into()),
                None => sea_orm::ActiveValue::not_set(),
            },
            most_recent_client: sea_orm::ActiveValue::not_set(),
            role: sea_orm::ActiveValue::not_set(),
            creation_time: sea_orm::ActiveValue::set(Local::now().into()),
            ..Default::default()
        };

        // applies the user active model
        let user: Result<users::Model, sea_orm::DbErr> =
            user.insert(&self.database_connection).await;

        // updates the username to be unique by adding the id of the user to the end `{username}-{id}`
        let user: Result<users::Model, sea_orm::DbErr> = match user {
            Ok(user_model) => {
                let mut user: users::ActiveModel = user_model.clone().into_active_model();
                user.set(
                    users::Column::Username,
                    format!("{}-{}", user_model.username, user_model.id).into(),
                );
                user.update(&self.database_connection).await
            }
            Err(user_err) => Err(user_err),
        };

        // catches any error prone db code and returns to the user
        match user {
            Err(error) => {
                return responses::user::CreateUserResponse::ERROR(Json(
                    json!({"error" : format!("{error:?}"), "code":500}),
                ));
            }

            Ok(user_model) => {
                client.user_name = user_model.username;
                return responses::user::CreateUserResponse::Ok(
                    Json(json!({
                        "message": format!("{} account has been created", name.clone().unwrap_or("".to_string())).as_str()
                    })),
                    client.to_cookie_string(&self.args, server_secret.0.clone(), None),
                );
            }
        }
    }

    /// User Edit
    ///
    /// # Edit Name
    /// This route is to remove or change the name of the user note this is not the same as
    /// username.
    #[oai(path = "/user/edit", method = "get", tag = ApiTags::User)]
    pub async fn wow(
        &self,
        auth: auth::ApiSecurityScheme,
        #[oai(name = "NewName")] username: Header<String>,
    ) -> responses::user::EditUserResponse {
        let user: Result<Option<users::Model>, sea_orm::DbErr> = users::Entity::find()
            .filter(users::Column::Username.contains(auth.0.user_name))
            .one(&self.database_connection)
            .await;

        match user {
            Ok(user) => match user {
                Some(user) => {
                    let mut new_user: users::ActiveModel = user.clone().into();
                    new_user.name = DataBaseSet(Some(username.0.clone()));
                    new_user.update(&self.database_connection).await.unwrap();
                    responses::user::EditUserResponse::Ok(Json(json!({
                        "message":
                            format!(
                                "User {}'s name was updated to {}",
                                user.username, username.0
                            )
                    })))
                }
                None => responses::user::EditUserResponse::ERROR(Json(
                    json!({"error" : format!("User was not found in databse."), "code":404}),
                )),
            },
            Err(error) => responses::user::EditUserResponse::ERROR(Json(
                json!({"error" : format!("{error:?}"), "code":500}),
            )),
        }

        //Json(json!({"Info": {"ActiveUserToken": auth.0, "Name": username.clone()}}))
    }

    /// Create A New Post/Note
    ///
    /// This route is to create A new post and returning a adquite response to user.
    #[oai(path = "/post/create", method = "put", tag = ApiTags::Post)]
    pub async fn post_create(
        &self,
        auth: auth::ApiSecurityScheme,
        req: PostCreation,
    ) -> PostCreationResponse {
        let body = match req {
            PostCreation::CreatePost(body) => body,
        };

        PostCreationResponse::PostCreated(Json(PostResponseSuccess {
            username: "coolname".to_string(),
            post_id: 10u64,
        }))
    }

    /// Edit An Exsiting Post/Note
    ///
    /// This route is to edit an existing post by `PostId`.
    #[oai(path = "/post/edit", method = "post", tag = ApiTags::Post)]
    pub async fn post_edit(
        &self,
        auth: auth::ApiSecurityScheme,
        #[oai(name = "PostId")] post_id: Header<String>,
        req: PostEdition,
    ) -> PostEditionResponse {
        PostEditionResponse::Forbiden
    }

    /// Delete A Post/Note
    ///
    /// This route is to delete a note by `PostId`.
    #[oai(path = "/post/delete", method = "delete", tag = ApiTags::Post)]
    pub async fn post_delete(
        &self,
        auth: auth::ApiSecurityScheme,
        req: PostSelection,
    ) -> PostDeletionResponse {
        PostDeletionResponse::Forbiden
    }

    /// Get A Post/Note
    ///
    /// This route gets posts/notes by id.
    #[oai(path = "/post/get", method = "get", tag = ApiTags::Post)]
    pub async fn post_get(&self, #[oai(name = "PostId")] post_id: Query<u64>) -> PostGetResponse {
        PostGetResponse::PostNotFound
    }
}
