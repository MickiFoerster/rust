use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        tx1.send("Hello").await.unwrap();
    });
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        tx2.send("World!").await.unwrap();
    });

    loop {
        let response = tokio::select! {
            Some(v) = rx1.recv() => {
                println!("Got {:?} from rx1", v);
                (chrono::Utc::now(), v)
            }
            Some(v) = rx2.recv() => {
                println!("Got {:?} from rx2", v);
                (chrono::Utc::now(), v)
            }
            else => {
                println!("Both channels closed");
                break;
            }
        };

        println!("{} received: {}", response.0, response.1);
    }
}
