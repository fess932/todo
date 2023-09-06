// use chrono::serde::ts_microseconds;
// use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
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
        id: "id1".to_owned(),
        name: message,
        status: "statuis1".to_owned(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub name: String,
}
