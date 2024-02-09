use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(3));
    interval.tick().await;
    println!("first tick is finished immediately");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("3 seconds timer elapsed");
            },
        }
    }
}
