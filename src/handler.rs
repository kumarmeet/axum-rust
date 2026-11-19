use crate::error::ApiError;
use crate::model::{NewPost, Post};
use crate::repo::PostRepo;
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

pub async fn get_post<R: PostRepo>(
    State(repo): State<R>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, ApiError> {
    let post = repo.get(id).await?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(ApiError::NotFound(id)),
    }
}

#[tracing::instrument(skip(repo))]
pub async fn create_post<R: PostRepo>(
    State(repo): State<R>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<(StatusCode, Json<Post>), ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let post = repo.create(p).await?;

    tracing::debug!(id = post.id, "created new post");

    // if let Ok(json) = serde_json::to_string_pretty(&p) {
    //     tracing::debug!(payload = %json, "Incoming payload");
    // }

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn update_post<R: PostRepo>(
    State(repo): State<R>,
    Path(id): Path<i64>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<Json<Post>, ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let post = repo.update(id, p).await?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(ApiError::NotFound(id)),
    }
}

pub async fn delete_post<R: PostRepo>(
    State(repo): State<R>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let result = repo.delete(id).await?;

    if !result {
        return Err(ApiError::NotFound(id));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_post<R: PostRepo>(State(repo): State<R>) -> Result<Json<Vec<Post>>, ApiError> {
    let posts = repo.list().await?;
    Ok(Json(posts))
}
