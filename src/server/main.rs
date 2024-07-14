use anyhow::Context;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Extension;
use axum::{http::StatusCode, routing::get, Json, Router};
use chrono::Utc;
use models::errors::{CustomError, CustomErrorEnum};
use models::message;
use models::message::{NewTask, Task, UpdateTask};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
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
        .route("/task/done", post(done_task))
        .route("/task/undone", post(undone_task))
        // `POST /users` goes to `create_user`
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3456").await.unwrap();
    println!("server running at :3456");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn new_task(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<NewTask>,
) -> Result<impl IntoResponse, CustomError> {
    let task = message::new_task(payload.name);
    let task2 = task.clone();

    // language=sqlite
    sqlx::query(
        "insert into task (id, created_at, updated_at, status, name) values ($1, $2, $3, $4, $5)"
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

async fn done_task(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, CustomError> {
    // language=sqlite
    let task = sqlx::query_as::<_, Task>(
        "update task set status='done', updated_at=$1 where id=$2 returning *",
    )
        .bind(Utc::now())
        .bind(payload.id)
        .fetch_one(&pool)
        .await
        .map_err(|err| CustomErrorEnum::TaskNotFound.text(err.to_string()))?;

    Ok((StatusCode::OK, Json(task)))
}

async fn undone_task(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, CustomError> {
    // language=sqlite
    let task = sqlx::query_as::<_, Task>(
        "update task set status='init', updated_at=$1 where id=$2 returning *",
    )
        .bind(Utc::now())
        .bind(payload.id)
        .fetch_one(&pool)
        .await
        .map_err(|err| CustomErrorEnum::TaskNotFound.text(err.to_string()))?;

    Ok((StatusCode::OK, Json(task)))
}

async fn list_task(Extension(pool): Extension<SqlitePool>) -> Result<Json<Vec<Task>>, CustomError> {
    // language=sqlite
    let task = sqlx::query_as::<_, Task>("select * from task")
        .fetch_all(&pool)
        .await
        .map_err(|err| CustomErrorEnum::TaskNotFound.text(err.to_string()))?;

    Ok(Json(task))
}
