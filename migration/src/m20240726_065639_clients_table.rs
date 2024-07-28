use super::m20220101_000001_users_table::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Clients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Clients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Clients::UserId).integer().not_null())
                    .col(ColumnDef::new(Clients::ClientIdentifier).char().not_null())
                    .col(ColumnDef::new(Clients::ClientSecret).char().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_users_id")
                            .from(Clients::Table, Clients::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Clients::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Clients {
    Table,
    Id,
    UserId,
    ClientIdentifier,
    ClientSecret,
}
