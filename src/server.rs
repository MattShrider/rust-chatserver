use axum::{routing::get, Router};

async fn get_index(_body: String) -> String {
    "neat".to_owned()
}

pub fn server() -> Router {
    Router::new().route("/", get(get_index))
}
