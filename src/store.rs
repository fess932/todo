use std::{env, str::FromStr};

use crate::models::{NewTask, Task};
use dotenvy::dotenv;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

#[derive(Clone)]
pub struct Store {
    pub pool: Pool<Sqlite>,
}

pub async fn get_connection_pool() -> Pool<Sqlite> {
    dotenv().ok();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::from_str(
                env::var("DATABASE_URL")
                    .expect("not found variable")
                    .as_str(),
            )
            .unwrap()
            .create_if_missing(true)
            .clone(),
        )
        .await
        .expect("msg")
}

impl Store {
    pub fn new_task(&self, _new_task: NewTask) -> Task {
        // let ta = Task::default();

        // let a = serde_json::to_vec(&ta).expect("failed to serialize");

        // let txn = self.db.begin_write().expect("msg");
        // {
        //     let mut table = txn.open_table(TABLE).expect("open table");
        //     table
        //         .insert("my_key", a.as_slice())
        //         .expect("failed to insert");
        // }
        // txn.commit().expect("failed to commit");

        // let read_txn = self.db.begin_read().expect("failed to open read txn");
        // let table = read_txn
        //     .open_table(TABLE)
        //     .expect("failed to open table for read");

        // let bb = table.get("my_key").unwrap().unwrap();
        // let tt: Task = serde_json::from_slice(bb.value()).expect("failed to form slice");
        // dbg!("table!", tt);

        Task::default()
    }
}
