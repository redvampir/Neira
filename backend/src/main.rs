use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Короткий баннер при старте — «примиряем» обе версии :)
    info!("Hello, world!");

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on http://{}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app).await {
        error!("server error: {err}");
    }
}
