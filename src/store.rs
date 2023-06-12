use std::sync::Arc;

use crate::models::{NewTask, Task};
use redb::{Database, ReadableTable, TableDefinition};

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");

pub fn connect() -> Database {
    Database::create("my_db.redb").expect("failed to open conn to db")
}

#[derive(Clone)]
pub struct Store {
    pub db: Arc<Database>,
}

impl Store {
    pub fn new_task(&self, _new_task: NewTask) -> Task {
        let txn = self.db.begin_write().expect("msg");
        {
            let mut table = txn.open_table(TABLE).expect("open table");
            table.insert("my_key", &123).expect("failed to insert");
        }
        txn.commit().expect("failed to commit");

        let read_txn = self.db.begin_read().expect("failed to open read txn");
        let table = read_txn
            .open_table(TABLE)
            .expect("failed to open table for read");
        println!("table!: {}", table.get("my_key").unwrap().unwrap().value());

        Task::default()
    }
}
