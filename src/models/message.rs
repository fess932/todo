// use chrono::serde::ts_microseconds;
// use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Task {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: String,
    pub name: String,
    pub status: String,
}

pub fn new_task(message: String) -> Task {
    Task {
        created_at: Utc::now(),
        updated_at: Utc::now(),
        id: Uuid::new_v4().to_string(),
        name: message,
        status: "init".to_owned(),
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct NewTask {
    pub name: String,
}
