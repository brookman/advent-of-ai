use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec};
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

pub type DB = Arc<Mutex<Database>>;

// pub fn todo_db() -> DB {
//     Arc::new(Mutex::new(Database {
//         todos: vec![],
//         ranking: vec![],
//     }))
// }

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub todos: Vec<Todo>,
    pub ranking: Vec<Contestant>,
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contestant {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub version: String,
    pub used_models_and_apis: Vec<String>,
}

