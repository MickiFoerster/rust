extern crate reqwest;
use reqwest::header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://example.com/v1/post-endpoint")
        .basic_auth("USER", Some("password"))
        .headers(headers)
        .body(r#"

      {
          "amount": "10.00",
          "description": "test",
      }
"#
        )
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
