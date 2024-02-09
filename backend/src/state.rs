use axum_macros::FromRef;
use sqlx::PgPool;

#[derive(FromRef, Clone)]
pub struct ServerState {
    db_pool: PgPool,
}

impl ServerState {
    pub(crate) fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
