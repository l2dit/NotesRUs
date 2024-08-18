//! # Notes R Us Migrations
//!
//! This crate is used to apply the required shema for [`notes_r_us`](https://crates.io/crates/notes_r_us). Also look [here](https://dbdocs.io/21ltietjens/Notes-R-Us) for a visual representation of
//! the database schema.
//!
//! ## Table Enums:
//!
//! | Diagram | Enum |
//! | - | - |
//! | [Users](https://dbdocs.io/21ltietjens/Notes-R-Us?table=users) | [`m20220101_000001_users_table::Users`] |
//! | [Clients](https://dbdocs.io/21ltietjens/Notes-R-Us?table=clients) | [`m20240726_065639_clients_table::Clients`] |
//! | [Follows](https://dbdocs.io/21ltietjens/Notes-R-Us?table=follows) | [`m20240727_035016_follows_table::Follows`] |
//! | [Posts](https://dbdocs.io/21ltietjens/Notes-R-Us?table=posts) | [`m20240727_061854_posts_table::Posts`] |
//! | [Comments](https://dbdocs.io/21ltietjens/Notes-R-Us?table=comments) | [`m20240727_063114_comments_table`] |
//! | [One Time Codes](https://dbdocs.io/21ltietjens/Notes-R-Us?table=one_time_codes) | [`m20240816_001629_otp_codes_table::OtpCodes`] |

pub use sea_orm_migration::prelude::*;

pub mod m20220101_000001_users_table;
pub mod m20240726_065639_clients_table;
pub mod m20240727_035016_follows_table;
pub mod m20240727_061854_posts_table;
pub mod m20240727_063114_comments_table;
pub mod m20240729_222557_foreign_keys;
pub mod m20240816_001629_otp_codes_table;

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
