mod server;
mod settings;

use server::server;
use settings::*;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let settings = config().expect("Loading config");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::from(&settings.log_level))
        .pretty()
        .init();

    tracing::debug!(?settings);

    let app = server().layer(TraceLayer::new_for_http());

    axum::Server::bind(
        &format!("{}:{}", &settings.host, &settings.port)
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
