use axum::{http::StatusCode, routing::get, Json, Router};
use models::message;
use models::message::{NewTask, Task};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(new_task))
        // `POST /users` goes to `create_user`
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn new_task(Json(payload): Json<NewTask>) -> (StatusCode, Json<Task>) {
    (StatusCode::CREATED, Json(message::new_task(payload.name)))
}

// basic handler that responds with a static string
async fn root() -> (StatusCode, Json<Task>) {
    (
        StatusCode::OK,
        Json(message::new_task("message".to_owned())),
    )
}
