use anyhow::Context;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{http::StatusCode, routing::get, Json, Router};
use models::errors::{CustomError, CustomErrorEnum};
use models::message;
use models::message::{NewTask, Task};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://target/wtf.db")
        .await
        .context("could not connect to database url")
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(list_task).post(new_task))
        // `POST /users` goes to `create_user`
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn new_task(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<NewTask>,
) -> Result<impl IntoResponse, CustomError> {
    let task = message::new_task(payload.name);
    let task2 = task.clone();

    sqlx::query(
        "INSERT INTO task (id, created_at, updated_at, status, name) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(task.id)
    .bind(task.created_at)
    .bind(task.updated_at)
    .bind(task.status)
    .bind(task.name)
    .execute(&pool)
    .await
    .map_err(|err| CustomErrorEnum::TaskNotFound.text(err.to_string()))?;

    Ok((StatusCode::CREATED, Json(task2)))
}

async fn list_task(Extension(pool): Extension<SqlitePool>) -> Result<Json<Vec<Task>>, CustomError> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM task")
        .fetch_all(&pool)
        .await
        .map_err(|err| CustomErrorEnum::TaskNotFound.text(err.to_string()))?;

    Ok(Json(task))
}
