use jwt::SignWithKey;
use poem::web::Data;
use poem_openapi::{param::Header, payload::Json, OpenApi, Tags};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use self::{
    auth::{ServerSecret, UserToken},
    requests::post::{PostCreation, PostDeletion, PostEdition},
    responses::post::{
        PostCreationResponse, PostDeletionResponse, PostEditionResponse, PostResponseSuccess,
    },
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Wow {
    cow: String,
    wow: String,
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
        }

        responses::user::CreateUserResponse::Ok(
            Json(json!({
                "message": format!("{} accont has not been created", name.clone().unwrap_or("".to_string())).as_str()
            })),
            client.to_cookie_string(&self.args, server_secret.0.clone(), None),
        )
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
        req: PostDeletion,
    ) -> PostDeletionResponse {
        PostDeletionResponse::Forbiden
    }
}
