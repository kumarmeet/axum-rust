use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

// #[derive(Clone, Debug, Serialize, Default)]
// pub struct Store {
//     pub posts: BTreeMap<u32, Post>,
//     pub next_id: u32,
// }

// every request may run on a different thread,
// The mutex lets only one handler touch the store at a time, and the Arc lets all handlers share it.
// pub type DB = Arc<Mutex<Store>>;
pub type DB = sqlx::SqlitePool;
