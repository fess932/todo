pub mod models;
pub mod schema;


use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

use uuid::{Uuid};

use crate::models::{NewTask, Status, Task};

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connect pool")
}

pub fn create_task(conn: &mut SqliteConnection, title: String, message: String) -> Task {
    use crate::schema::task;

    let new_post = NewTask {
        id: Uuid::new_v4().to_string(),
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
        status: Status::Init.as_str(),
        title: title.parse().unwrap(),
        message: message.parse().unwrap(),
    };

    diesel::insert_into(task::table)
        .values(&new_post)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
