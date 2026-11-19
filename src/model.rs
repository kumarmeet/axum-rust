use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Default, sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub body: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewComment {
    pub body: String,
}

// every request may run on a different thread,
// The mutex lets only one handler touch the store at a time, and the Arc lets all handlers share it.
// pub type DB = Arc<Mutex<Store>>;
pub type DB = sqlx::SqlitePool;
