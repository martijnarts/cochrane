use sqlx::PgPool;

use crate::config::DatabaseConfig;

pub async fn connect_to_postgres(settings: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let connection = PgPool::connect_with(settings.as_connect_options()).await?;
    Ok(connection)
}
