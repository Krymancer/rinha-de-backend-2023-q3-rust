use std::net::SocketAddr;
use axum::{
    Router, 
    routing::{post, get}, 
    response::{IntoResponse, Response}, 
    http::{StatusCode, header},
    extract::{Path, Query, Json},
    Extension
};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use serde_json::json;
use anyhow::Context;
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const MAX_CONNECTIONS : u32 = 50;
    const DATABASE_URL : &str = "postgres://user:password@db/db";

    std::env::set_var("RUST_LOG", "debug");
    print!("Creating Pg pool...");

    let db = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&DATABASE_URL)
        .await
        .context("Failed to connect to Postgres")?;

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

/* Handlers */
async fn post_pessoas(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<CreatePessoa>
) -> Response {
    let uuid = uuid::Uuid::new_v4().to_string();

    let date = NaiveDate::parse_from_str(&payload.nascimento, "%Y-%m-%d");

    let date = match date {
        Ok(date) => date,
        Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY).into_response()
    };

    let stack = match payload.stack {
        Some(stack) => stack,
        None => vec![]
    };

    let stack =  stack.into_iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(",");

    let pessoa = Pessoa {
        id: uuid,
        apelido: payload.apelido,
        nome: payload.nome,
        nascimento: date.to_string(),
        stack
    };

    let result = sqlx::query("INSERT INTO pessoas (id, apelido, nome, nascimento, stack) VALUES ($1, $2, $3, $4, $5)")
    .bind(&pessoa.id)
    .bind(&pessoa.apelido)
    .bind(&pessoa.nome)
    .bind(&pessoa.nascimento)
    .bind(&pessoa.stack)
    .execute(&db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, [(header::LOCATION, format!("/pessoas/{}", pessoa.id))]).into_response(),
        Err(sqlx::Error::Database(_)) => (StatusCode::UNPROCESSABLE_ENTITY).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

async fn get_pessoas_id(
    Extension(db): Extension<PgPool>,
    Path(id): Path<String>
) -> Response {
    let result = sqlx::query("SELECT id, apelido, nome, nascimento, stack FROM pessoas WHERE id = $1 LIMIT 1")
    .bind(id)
    .fetch_optional(&db)
    .await;

    match result {
        Ok(Some(pessoa)) => return (StatusCode::OK, Json(json!({"pessoa": Pessoa::from_pg_row(pessoa)}))).into_response(),
        Ok(None) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    };
}

async fn get_pessoas_busca(
    Extension(db): Extension<PgPool>,
    Query(query): Query<QueryPessoa>
) -> Response {
    let search_term  = query.t;
    
    let result = sqlx::query("SELECT id, apelido FROM PESSOAS
    WHERE BUSCA ILIKE $1 LIMIT 50;")
    .bind(format!("%{search_term}"))
    .fetch_all(&db)
    .await;

    let pessoas = match result {
        Ok(pessoas) => pessoas.into_iter().map(|p| PessoaResponse::from_pg_row(p)).collect::<Vec<_>>(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    };

    (StatusCode::OK, Json(json!(pessoas))).into_response()
}

async fn contagem_pessoas(
    Extension(db): Extension<PgPool>
) -> Response {
    let result = sqlx::query("SELECT COUNT(*) FROM pessoas")
    .fetch_one(&db)
    .await;

    match result {
        Ok(row) => return (StatusCode::OK, Json(json!({"count": row.get::<i64, usize>(0)}))).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    };
}

/* Models Requests ETC */
#[derive (Debug, Serialize, Deserialize )]
struct Pessoa {
    id: String,
    apelido: String,
    nome: String,
    nascimento: String,
    stack: String
}

#[derive (Debug, Serialize, Deserialize )]
struct PessoaResponse {
    id: String,
    apelido: String,
    nome: String,
    nascimento: String,
    stack: Option<Vec<String>>
}

#[derive (Debug, Serialize, Deserialize )]
struct CreatePessoa {
    apelido: String,
    nome: String,
    nascimento: String,
    stack: Option<Vec<String>>
}

#[derive (Debug, Serialize, Deserialize )]
struct QueryPessoa {
    t: String
}

impl PessoaResponse {
    fn from_pg_row(row: sqlx::postgres::PgRow) -> Self {
        let stack_list: Option<String> = row.get(4);

        let stack = match  stack_list {
            Some(stack) => Some(stack.split(',').map(|s| s.to_string()).collect::<Vec<String>>()),
            None => None
        };

        Self {
            id: row.get(0),
            apelido: row.get(1),
            nome: row.get(2),
            nascimento: row.get(3),
            stack: stack
        }
    }
}

impl Pessoa {
    fn from_pg_row(row: sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get(0),
            apelido: row.get(1),
            nome: row.get(2),
            nascimento: row.get(3),
            stack: row.get(4)
        }
    }
}
