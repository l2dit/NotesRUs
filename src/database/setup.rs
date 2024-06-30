use sea_orm::*;

/// Connects to data base and insures the database exsits.
/// Suports [mysql, postgresql, sqlite].
pub async fn set_up_db(
    database_url: &str,
    database_name: &str,
) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await.unwrap();

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", database_name),
            ))
            .await
            .expect("database exsits");

            let url = format!("{}/{}", database_url, database_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}
