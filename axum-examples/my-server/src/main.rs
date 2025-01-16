use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    use reqwest::header::USER_AGENT;

    let client = reqwest::Client::new();
    let res = client
        .get("https://www.google.com/")
        .header(USER_AGENT, "example agent")
        .send()
        .await
        .unwrap();
    println!("status: {}", res.status());
    println!("headers:");
    println!("{:#?}", res.headers());

    let body = res.text().await.unwrap();

    Html(body)
}
