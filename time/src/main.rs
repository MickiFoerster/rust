use chrono::{Duration, Utc};

fn main() {
    println!("Hello, world at now {}!", Utc::now());
    println!(
        "Hello, world in 1h {}!",
        Utc::now() + Duration::seconds(3600)
    );
}
