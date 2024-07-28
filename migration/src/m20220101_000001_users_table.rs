use super::m20240726_065639_clients_table::Clients;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).char().not_null())
                    .col(ColumnDef::new(Users::Name).char().not_null())
                    .col(ColumnDef::new(Users::MostRecentClient).integer())
                    .col(ColumnDef::new(Users::Role).integer().default(0u8))
                    .col(
                        ColumnDef::new(Users::CreationTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_most_recently_used_client")
                            .from(Users::Table, Users::MostRecentClient)
                            .to(Clients::Table, Clients::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Name,
    MostRecentClient,
    Role,
    CreationTime,
}
