use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, SET_COOKIE};
use axum::{
    response::{AppendHeaders, IntoResponse, Redirect, Response},
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use http::StatusCode;
use serde_json::json;
use std::net::SocketAddr;

mod login;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    tracing::debug!("CLIENT_ID: {}", std::env::var("CLIENT_ID").unwrap());
    tracing::debug!("CLIENT_SECRET: {}", std::env::var("CLIENT_SECRET").unwrap());
    tracing::debug!("ISSUER: {}", std::env::var("ISSUER").unwrap());

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/login") }))
        .route("/login", get(login))
        .route("/redirect-endpoint", get(redirect_endpoint))
        .route("/logout", get(logout));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=UTF-8")],
        login::LOGIN_PAGE_HTML.into_response(),
    )
}

async fn redirect_endpoint() -> impl IntoResponse {
    use reqwest::header::USER_AGENT;

    let client = reqwest::Client::new();
    let res = client
        .get("https://www.google.com/")
        .header(USER_AGENT, "example agent")
        .send()
        .await
        .unwrap();
    tracing::debug!("status: {}", res.status());
    tracing::debug!("headers:");
    tracing::debug!("{:#?}", res.headers());

    (
        AppendHeaders([
            (CACHE_CONTROL, "no-cache"),
            (SET_COOKIE, "mycookie=mycookievalue"),
        ]),
        res.text().await.unwrap(),
    )
}
async fn logout() -> String {
    "".to_string()
}
