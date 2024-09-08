use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};

use crate::{
    backend::{auth::security_scheme::UserToken, responses::user},
    entity::{clients, users},
};

pub struct CheckAuth {
    pub database_connection: DatabaseConnection,
    pub user_token: UserToken,
    pub user_id: Option<i32>,
}

impl CheckAuth {
    pub async fn get_user_model(&mut self) -> Result<Option<users::Model>, DbErr> {
        let user_id = self.get_user_id().await;

        match user_id {
            Ok(option_user_id) => match option_user_id {
                None => return Ok(None),
                Some(ref user_id) => {
                    return users::Entity::find_by_id(*user_id)
                        .one(&self.database_connection)
                        .await;
                }
            },
            Err(error) => return Err(error),
        };
    }

    pub async fn set_recent_client(&mut self) -> Result<(), DbErr> {
        match &self.user_id {
            None => {
                let _ = match self.get_user_id().await {
                    Ok(Some(_)) => Ok(()),
                    Err(error) => Err(error),
                    Ok(None) => Err(DbErr::Custom("user not found".to_string())),
                };
            }
            _ => (),
        };

        let user = users::Entity::find_by_id(self.user_id.unwrap())
            .one(&self.database_connection)
            .await;

        let client = clients::Entity::find()
            .filter(clients::Column::ClientIdentifier.contains(&self.user_token.client_identifier))
            .one(&self.database_connection)
            .await;

        match user {
            Ok(Some(user_model)) => match client {
                Ok(Some(client_model)) => {
                    let mut user_active_model = user_model.into_active_model();
                    user_active_model.set(users::Column::MostRecentClient, client_model.id.into());

                    return match user_active_model.update(&self.database_connection).await {
                        Ok(_) => Ok(()),
                        Err(error) => Err(error),
                    };
                }
                _ => Err(DbErr::Custom(
                    "Client Not Found/Internal Database Error".to_string(),
                )),
            },
            _ => Err(DbErr::Custom(
                "User Not Found/Internal Database Error".to_string(),
            )),
        }
    }

    pub async fn get_user_id(&mut self) -> Result<Option<i32>, DbErr> {
        let client: Result<Option<clients::Model>, DbErr> = clients::Entity::find()
            .filter(clients::Column::ClientIdentifier.contains(&self.user_token.client_identifier))
            .filter(clients::Column::ClientSecret.contains(&self.user_token.client_secret))
            .one(&self.database_connection)
            .await;

        match client {
            Ok(option_model) => Ok(match option_model {
                Some(model) => {
                    self.user_id = Some(model.user_id);
                    Some(model.user_id)
                }
                None => None,
            }),
            Err(error) => Err(error),
        }
    }
}
