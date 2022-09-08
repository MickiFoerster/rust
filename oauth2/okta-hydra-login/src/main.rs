use axum::http::header::SET_COOKIE;
use axum::{
    response::{AppendHeaders, IntoResponse, Redirect},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/login") }))
        .route("/login", get(login))
        .route("/redirect-endpoint", get(redirect_endpoint))
        .route("/logout", get(logout));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login() -> impl IntoResponse {
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

    (
        AppendHeaders([(SET_COOKIE, "foo=bar"), (SET_COOKIE, "baz=qux")]),
        res.text().await.unwrap(),
    )
}

async fn redirect_endpoint() -> String {
    "".to_string()
}
async fn logout() -> String {
    "".to_string()
}
