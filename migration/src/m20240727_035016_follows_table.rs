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
                    .table(Follows::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Follows::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Follows::FollowingUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Follows::FollowedUserId).integer().not_null())
                    .col(
                        ColumnDef::new(Follows::CreationTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid_of_following_user")
                            .from(Follows::Table, Follows::FollowingUserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid_of_followed_user")
                            .from(Follows::Table, Follows::FollowedUserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Follows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Follows {
    Table,
    Id,
    FollowingUserId,
    FollowedUserId,
    CreationTime,
}
