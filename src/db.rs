#[cfg(feature = "ssr")]
use sqlx::{Connection, SqliteConnection};
#[cfg(feature = "ssr")]
use std::env::var;
#[cfg(feature = "ssr")]
use leptos::ServerFnError;

#[cfg(feature = "ssr")]
pub async fn db() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(&var("DATABASE_URL")?).await?)
}

#[cfg(feature = "ssr")]
pub async fn init_db() -> Result<(), ServerFnError> {

    let mut conn = db().await?;

    let _ = sqlx::query!("CREATE TABLE IF NOT EXISTS guestbook (name text, message text)")
        .execute(&mut conn)
        .await?;

    Ok(())
}
