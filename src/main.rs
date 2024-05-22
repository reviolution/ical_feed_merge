extern crate axum;

use axum::extract::Path;
use axum::routing::get;
use axum::Router;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/:txt", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Path(txt): Path<String>) -> String {
    txt
}
