use axum::{response::IntoResponse, routing::get, Router};

pub fn room_routes() -> Router {
    Router::new().route("/", get(get_room_root))
}

async fn get_room_root() -> impl IntoResponse {
    "Rooms not yet implemented"
}
