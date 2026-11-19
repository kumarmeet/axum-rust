use crate::error::ApiError;
use crate::model::{DB, NewPost, Post};
use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;

fn check(new: &NewPost) -> Result<(), ApiError> {
    if new.title.is_empty() {
        return Err(ApiError::EmptyTitle);
    }
    Ok(())
}

pub async fn hello() -> &'static str {
    "hello world!"
}

pub async fn get_post(State(db): State<DB>, Path(id): Path<i64>) -> Result<Json<Post>, ApiError> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(&id)
        .fetch_one(&db)
        .await?;

    Ok(Json(post))
}

#[tracing::instrument(skip(db))]
pub async fn create_post(
    State(db): State<DB>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<(StatusCode, Json<Post>), ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let post =
        sqlx::query_as::<_, Post>("INSERT INTO posts (title, body) values (?, ?) returning *")
            .bind(p.title.clone())
            .bind(p.body.clone())
            .fetch_one(&db)
            .await?;

    tracing::debug!(id = post.id, "created new post");

    if let Ok(json) = serde_json::to_string_pretty(&p) {
        tracing::debug!(payload = %json, "Incoming payload");
    }

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn update_post(
    State(db): State<DB>,
    Path(id): Path<i64>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<Json<Post>, ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let post =
        sqlx::query_as::<_, Post>("UPDATE posts SET title = ?, body = ? WHERE id = ? returning *")
            .bind(&p.title)
            .bind(&p.body)
            .bind(id)
            .fetch_one(&db)
            .await?;

    Ok(Json(post))
}

pub async fn delete_post(
    State(db): State<DB>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(id));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_post(State(db): State<DB>) -> Result<Json<Vec<Post>>, ApiError> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY id")
        .fetch_all(&db)
        .await?;

    Ok(Json(posts))
}
