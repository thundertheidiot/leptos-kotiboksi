#[cfg(feature = "ssr")]
use sqlx::{Connection, SqliteConnection, migrate::MigrateDatabase, Sqlite};
#[cfg(feature = "ssr")]
use std::env::var;
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub async fn db() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(&var("DATABASE_URL")?).await?)
}

#[cfg(feature = "ssr")]
pub async fn init_db() -> Result<(), ServerFnError> {
    let url = &var("DATABASE_URL")?;
    if !Sqlite::database_exists(url).await? {
	Sqlite::create_database(url).await?;
    }

    let mut conn = db().await?;

    sqlx::migrate!().run(&mut conn).await?;

    Ok(())
}
