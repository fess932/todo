use chrono::serde::ts_microseconds;
use chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Task {
    #[serde(with = "ts_microseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    updated_at: DateTime<Utc>,
    id: String,
    name: String,
    status: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewTask {
    pub name: String,
}
