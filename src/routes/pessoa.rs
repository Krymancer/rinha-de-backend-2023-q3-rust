use axum::{Extension, response::{Response, IntoResponse}, Json, http::{StatusCode, header}, extract::{Path, Query}};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{requests::{CreatePessoa, QueryPessoa}, models::Pessoa};

pub async fn post_pessoas(
  Extension(db): Extension<PgPool>,
  Json(payload): Json<CreatePessoa>,
) -> Response {
  let uuid = uuid::Uuid::new_v4();
  let date = NaiveDate::parse_from_str(&payload.nascimento, "%Y-%m-%d");
  let date = match date {
      Ok(date) => date,
      Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY).into_response(),
  };

  match payload.stack {
      Some(ref stack) if stack.iter().any(|item| item.len() > 32) => {
          return (StatusCode::UNPROCESSABLE_ENTITY).into_response()
      }
      Some(_) => {}
      None => {}
  };

  let pessoa = Pessoa {
      id: uuid,
      apelido: payload.apelido,
      nome: payload.nome,
      nascimento: date.to_string(),
      stack: payload.stack,
  };

  let result = sqlx::query!(
      "INSERT INTO pessoa (id, apelido, nome, nascimento, stack) VALUES ($1, $2, $3, $4, $5)",
      pessoa.id,
      pessoa.apelido,
      pessoa.nome,
      pessoa.nascimento,
      pessoa.stack.as_ref().map(|stack| stack.as_slice())
  )
  .execute(&db)
  .await;

  match result {
      Ok(_) => (
          StatusCode::CREATED,
          [(header::LOCATION, format!("/pessoas/{}", pessoa.id))],
      )
          .into_response(),
      Err(sqlx::Error::Database(_)) => (StatusCode::UNPROCESSABLE_ENTITY).into_response(),
      Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
  }
}

pub async fn get_pessoas_id(Extension(db): Extension<PgPool>, Path(id): Path<Uuid>) -> Response {
  let result = sqlx::query_as!(
      Pessoa,
      "SELECT id, apelido, nome, nascimento, stack FROM pessoa WHERE id = $1 LIMIT 1",
      id
  )
  .fetch_optional(&db)
  .await;

  match result {
      Ok(Some(pessoa)) => (StatusCode::OK, Json(json!(pessoa))).into_response(),
      Ok(None) => (StatusCode::NOT_FOUND).into_response(),
      Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
  }
}

pub async fn get_pessoas_busca(
  Extension(db): Extension<PgPool>,
  Query(query): Query<QueryPessoa>,
) -> Response {
  let search_term = query.t;

  let result = sqlx::query_as!(
      Pessoa,
      "SELECT id, apelido, nome, nascimento, stack FROM pessoa WHERE search ILIKE $1 LIMIT 50;",
      format!("%{}%", search_term)
  )
  .fetch_all(&db)
  .await;

  match result {
      Ok(pessoas) => (StatusCode::OK, Json(json!(pessoas))).into_response(),
      Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
  }
}

pub async fn contagem_pessoas(Extension(db): Extension<PgPool>) -> Response {
  let result = sqlx::query_scalar!("SELECT COUNT(*) FROM pessoa")
      .fetch_one(&db)
      .await;

  match result {
      Ok(count) => (StatusCode::OK, Json(json!({"count": count}))).into_response(),
      Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
  }
}