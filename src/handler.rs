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

pub async fn get_post(State(db): State<DB>, Path(id): Path<u32>) -> Result<Json<Post>, ApiError> {
    let store = db.lock().unwrap();
    let post = store.posts.get(&id).ok_or(ApiError::NotFound(id))?;
    Ok(Json(post.clone()))
}

pub async fn create_post(
    State(db): State<DB>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<(StatusCode, Json<Post>), ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let mut store = db.lock().unwrap();
    store.next_id += 1;

    let post = Post {
        id: store.next_id,
        title: p.title,
        body: p.body,
    };

    store.posts.insert(post.id, post.clone());
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn update_post(
    State(db): State<DB>,
    Path(id): Path<u32>,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> Result<Json<Post>, ApiError> {
    let Json(p) = payload?;

    check(&p)?;

    let mut store = db.lock().unwrap();

    // get_mut return mutable reference
    let post = store.posts.get_mut(&id).ok_or(ApiError::NotFound(id))?;

    post.body = p.body;
    post.title = p.title;

    Ok(Json(post.clone()))
}

pub async fn delete_post(
    State(db): State<DB>,
    Path(id): Path<u32>,
) -> Result<StatusCode, ApiError> {
    let mut store = db.lock().unwrap();
    store.posts.remove(&id).ok_or(ApiError::NotFound(id))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_post(State(db): State<DB>) -> Json<Vec<Post>> {
    let store = db.lock().unwrap();
    Json(store.posts.values().cloned().collect())
}
