use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpClient, Transport};
use lettre_email::Email;

fn main() {
    // Create an email
    let email = Email::builder()
        .to("recipient@example.com")
        .from("sender@example.com")
        .subject("Test email")
        .text("Hello from Rust!")
        .build()
        .unwrap();

    // Connect to the SMTP server
    let smtp_server = "smtp.example.com";
    let smtp_port = 587;

    let smtp_username = "your_username";
    let smtp_password = "your_password";

    let smtp_client = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(Credentials::new(smtp_username, smtp_password))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Login)
        .transport();

    // Send the email
    let result = smtp_client.send(email.into());

    if result.is_ok() {
        println!("Email sent successfully!");
    } else {
        println!("Failed to send email: {:?}", result);
    }
}
