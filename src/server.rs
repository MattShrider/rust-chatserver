mod rooms;

use axum::{response::IntoResponse, routing::get, Router};
use hyper::StatusCode;

async fn get_root(_body: String) -> impl IntoResponse {
    "neat"
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "That page doesn't exist!")
}

pub fn server() -> Router {
    Router::new()
        .route("/", get(get_root))
        .nest("/rooms", rooms::room_routes())
        .fallback(not_found)
}
