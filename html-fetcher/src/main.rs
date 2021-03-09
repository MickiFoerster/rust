extern crate hyper;

use hyper::Client;

fn main() {
    println!("HELLO");
}
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//    let client = Client::new();
//
//    let uri = "http://www.com-science.de/".parse()?;
//
//    let rep = client.get(uri).await?;
//
//    println!("Response: {}", resp.status());
//
//    Ok(())
//}
