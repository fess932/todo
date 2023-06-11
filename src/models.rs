use super::schema::task;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use std::fmt::Debug;

pub enum Status {
    Init,
    Done,
}

impl Status {
    pub fn as_str(&self) -> String {
        match self {
            Status::Init => String::from("init"),
            Status::Done => String::from("done"),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = task)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub status: String,
    pub title: String,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask {
    pub id: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub status: String,
    pub title: String,
    pub message: String,
}
