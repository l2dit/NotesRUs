use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

use crate::{m20220101_000001_users_table::Users, m20240726_065639_clients_table::Clients};

/// Migration Defintion
#[derive(DeriveMigrationName)]
pub struct Migration;

/// Migration Implementation
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// User Table Foreign Key Def
    /// This Is Reqired Because sqlite doesen't like table to be edited therfore is done at table defiton. However Postgres Wants the table thats refranced to exist before making it a foreign key so its done here.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let fk_most_recent_client = TableForeignKey::new()
            .name("FK_most_recent_client")
            .from_tbl(Users::Table)
            .from_col(Users::MostRecentClient)
            .to_tbl(Clients::Table)
            .to_col(Users::Id)
            .to_owned();

        if manager.get_database_backend() != DbBackend::Sqlite {
            return manager
                .alter_table(
                    Table::alter()
                        .table(Users::Table)
                        .add_foreign_key(&fk_most_recent_client)
                        .to_owned(),
                )
                .await;
        } else {
            Ok(())
        }
    }
}
