use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Store {
    pub posts: BTreeMap<u32, Post>,
    pub next_id: u32,
}

// every request may run on a different thread,
// The mutex lets only one handler touch the store at a time, and the Arc lets all handlers share it.
pub type DB = Arc<Mutex<Store>>;
