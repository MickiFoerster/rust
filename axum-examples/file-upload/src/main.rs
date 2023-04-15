use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(show_form).post(accept_form));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_form() -> impl IntoResponse {
    Html(format!(
        r#"
        <html>
        <body>
        
<form action="/" method="post" enctype="multipart/form-data">
<label for="avatar">Choose a profile picture:</label>

<input type="file"
       id="avatar" name="avatar"
       accept="image/png, image/jpeg">

<input type="submit" value="Upload">
        </form>

        </body>
        </html>
            "#
    ))
    .into_response()
}

async fn accept_form(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
        std::fs::write("/tmp/upload", data).expect("write to file failed");
    }
}
