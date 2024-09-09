use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Value,
};

use crate::{
    backend::auth::security_scheme::UserToken,
    entity::{clients, users},
};

#[derive(Debug)]
pub struct CheckAuth {
    database_connection: DatabaseConnection,
    pub client_id: i32,
    pub user_id: i32,
    pub user_model: Option<users::Model>,
}

impl CheckAuth {
    pub async fn new(
        database_connection: DatabaseConnection,
        user_token: UserToken,
    ) -> Result<Self, DbErr> {
        let client: Result<Option<clients::Model>, DbErr> = clients::Entity::find()
            .filter(clients::Column::ClientIdentifier.contains(&user_token.client_identifier))
            .filter(clients::Column::ClientSecret.contains(&user_token.client_secret))
            .one(&database_connection)
            .await;

        match client {
            Ok(Some(client_model)) => Ok(CheckAuth {
                database_connection,
                client_id: client_model.id,
                user_id: client_model.user_id,
                user_model: None,
            }),
            Ok(None) => Err(DbErr::Custom(
                "Client Was Not Found in The Database".to_string(),
            )),
            Err(error) => Err(error),
        }
    }
    pub async fn find_user_model(mut self) -> Result<Self, DbErr> {
        let user: Result<Option<users::Model>, DbErr> = users::Entity::find_by_id(self.user_id)
            .one(&self.database_connection)
            .await;

        match user {
            Ok(Some(user_model)) => {
                self.user_model = Some(user_model);
                Ok(self)
            }
            Ok(None) => Err(DbErr::Custom("User Not Found in Database".to_string())),
            Err(error) => Err(error),
        }
    }
    pub async fn log_client(mut self) -> Result<Self, DbErr> {
        if self.user_model == None {
            match self.find_user_model().await {
                Ok(self_) => self = self_,
                Err(error) => return Err(error),
            };
        }

        match self.user_model.clone() {
            Some(user_model) => {
                let mut user = user_model.clone().into_active_model();
                user.set(users::Column::MostRecentClient, Value::from(self.client_id));

                match user.update(&self.database_connection).await {
                    Ok(_) => Ok(self),
                    Err(error) => Err(error),
                }
            }
            None => Err(DbErr::Custom("User Not Found in Database".to_string())),
        }
    }
}
