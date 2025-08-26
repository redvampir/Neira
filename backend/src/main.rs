use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Сообщение в консоль при старте — «примиряем» обе версии :)
    println!("Hello, world!");

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
