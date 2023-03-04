use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[tokio::main]
async fn main() {
    println!("Sending email ...");
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

    let from = "John Doo <john.doo@example.com>";
    let to = "Jane.Doo@example.com";
    let subject = "Hello Jane";
    let body = "Hello Jane!".to_string();

    let email = Message::builder()
        .from(from.parse().expect("parse error"))
        .to(to.parse().expect("parse error"))
        .subject(subject)
        .body(body.to_string())
        .expect("parse error");

    mailer.send(email).await.expect("send error");
}
