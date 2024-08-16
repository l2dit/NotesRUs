use sea_orm_migration::prelude::*;

use crate::m20220101_000001_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OtpCodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OtpCodes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OtpCodes::UserId).char().not_null())
                    .col(ColumnDef::new(OtpCodes::Code).char().not_null())
                    .col(ColumnDef::new(OtpCodes::ExpiryDate).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid")
                            .from(OtpCodes::Table, OtpCodes::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OtpCodes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OtpCodes {
    Table,
    Id,
    UserId,
    Code,
    ExpiryDate,
}
