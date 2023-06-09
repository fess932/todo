use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    title: Cow<'static, str>,
    name: Name,
    marketing: bool,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = surrealdb::engine::any::connect("file://temp.db").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person with a random ID
    let created: Person = db
        .create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?;

    // Create a new person with a specific ID
    let created: Person = db
        .create(("person", "jaime"))
        .content(Person {
            title: "Founder & COO".into(),
            name: Name {
                first: "Jaime".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: false,
        })
        .await?;

    // Update a person record with a specific ID
    let updated: Person = db
        .update(("person", "jaime"))
        .merge(json!({"marketing": true}))
        .await?;

    // Select all people records
    let people: Vec<Person> = db.select("person").await?;

    // Perform a custom advanced query
    let sql = r#"
        SELECT marketing, count()
        FROM type::table($table)
        GROUP BY marketing
    "#;

    let groups = db.query(sql).bind(("table", "person")).await?;

    Ok(())
}
