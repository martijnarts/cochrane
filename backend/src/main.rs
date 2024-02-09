mod config;
mod database;
mod state;

use axum::{extract::State, http::Method, routing::get, Json, Router};
use axum_macros::debug_handler;
use config::get_configuration;
use database::connect_to_postgres;
use serde::Serialize;
use sqlx::{query, PgPool};
use state::ServerState;

#[derive(Serialize)]
struct GetResult {
    foo: String,
}

#[debug_handler]
async fn get_data(State(pool): State<PgPool>) -> Json<GetResult> {
    let res = query!("SELECT 5 + 5 AS sum;")
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(GetResult {
        foo: res.sum.unwrap_or_default().to_string(),
    })
}

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Could not load configuration");
    let db_pool = connect_to_postgres(config.database())
        .await
        .expect("Could not connect to database");

    let state = ServerState::new(db_pool);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/", get(get_data))
        .layer(cors)
        .with_state(state);

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
