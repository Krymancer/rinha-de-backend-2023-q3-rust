use anyhow::Context;
use axum::{
    Extension, Router, routing::{post, get},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

use api_rust_rinha_back::routes::pessoa::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const MAX_CONNECTIONS: u32 = 300;
    const DATABASE_URL: &str = "postgres://user:password@db/db";

    let db = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_URL)
        .await
        .context("Failed to connect to database Postgres")?;

    let app = Router::new()
        .route("/pessoas", post(post_pessoas).get(get_pessoas_busca))
        .route("/pessoas/:id", get(get_pessoas_id))
        .route("/contagem-pessoas", get(contagem_pessoas))
        .layer(Extension(db));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Server failed")
}