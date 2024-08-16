pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users_table;
mod m20240726_065639_clients_table;
mod m20240727_035016_follows_table;
mod m20240727_061854_posts_table;
mod m20240727_063114_comments_table;
mod m20240729_222557_foreign_keys;
mod m20240816_001629_otp_codes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users_table::Migration),
            Box::new(m20240726_065639_clients_table::Migration),
            Box::new(m20240727_035016_follows_table::Migration),
            Box::new(m20240727_061854_posts_table::Migration),
            Box::new(m20240727_063114_comments_table::Migration),
            Box::new(m20240729_222557_foreign_keys::Migration),
            Box::new(m20240816_001629_otp_codes_table::Migration),
        ]
    }
}
