use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_users_table::Users, m20240727_061854_posts_table::Posts};

/// Migration Defintion
#[derive(DeriveMigrationName)]
pub struct Migration;

/// Migration Implementation
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Comment Table Migration Up
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::UserId).integer().not_null())
                    .col(ColumnDef::new(Comments::PostId).integer().not_null())
                    .col(
                        ColumnDef::new(Comments::ReplyCommentId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comments::Body).text().not_null())
                    .col(
                        ColumnDef::new(Comments::CreationTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comments::EditTime).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userid")
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_postid")
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_reply_comment_id")
                            .from(Comments::Table, Comments::ReplyCommentId)
                            .to(Comments::Table, Comments::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    /// Comment Table Migration Down
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

/// Comments Table
#[derive(DeriveIden)]
pub enum Comments {
    Table,
    /// Comment Identifier
    Id,
    /// User That Created The Comment
    UserId,
    /// The Post The Comment Is Mentioning
    PostId,
    /// If Comment Is In Response To Another Comment The Id Of That Comment
    ReplyCommentId,
    /// Body Of The Comment
    Body,
    /// Timestamp Of The Intial Creation Of The Comment
    CreationTime,
    /// If Edited The Timestamp Of The Last Edit
    EditTime,
}
