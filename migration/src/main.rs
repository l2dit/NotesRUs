use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(notes_r_us_migrations::Migrator).await;
}
