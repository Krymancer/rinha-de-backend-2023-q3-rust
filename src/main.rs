use std::net::SocketAddr;
use axum::{
    Router, 
    routing::{post, get}, 
    response::{Response, IntoResponse, Json}, 
    http::StatusCode,
    Extension
};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;
use serde_json::json;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const MAX_CONNECTIONS : u32 = 10;
    const DATABASE_URL : &str = "postgres://user:password@db/db";

    let db = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&DATABASE_URL)
        .await
        .context("Failed to connect to Postgres")?;

    let app = Router::new()
    .route("/pessoas", post(post_pessoas).get(get_pessoas_busca))
    .route("/pessoas/:id", get(get_pessoas_id))
    .route("/contagem-pessoas", get(contagem_pessoas))
    .with_state(Extension(db));

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .context("Server failed")
}


/* Handlers */
async fn post_pessoas(
) -> Response {
    (StatusCode::OK, Json(json!({"message": "ok"}))).into_response()
}

async fn get_pessoas_id(
) -> Response {
    (StatusCode::OK, Json(json!({"message": "ok"}))).into_response()
}

async fn get_pessoas_busca() -> Response {
    (StatusCode::OK, Json(json!({"message": "ok"}))).into_response()
}

async fn contagem_pessoas() -> Response {
    (StatusCode::OK, Json(json!({"message": "ok"}))).into_response()
}


/* Models Requests ETC */
#[derive (Debug, Serialize, Deserialize )]
struct Pessoa {
    apelid: String,
    nome: String,
    nascimento: String,
    stack: Vec<String>
}

