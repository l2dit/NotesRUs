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
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::UserId).integer().not_null())
                    .col(ColumnDef::new(Posts::UpVotes).integer().default(0u8))
                    .col(ColumnDef::new(Posts::DownVotes).integer().default(0u8))
                    .col(ColumnDef::new(Posts::Title).char().not_null())
                    .col(ColumnDef::new(Posts::Body).text().not_null())
                    .col(
                        ColumnDef::new(Posts::CreationTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Posts::EditTime).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Posts {
    Table,
    Id,
    UserId,
    UpVotes,
    DownVotes,
    Title,
    Body,
    CreationTime,
    EditTime,
}
