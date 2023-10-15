#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::oneshot::channel(); 

    let task1 = tokio::spawn(async move {
        if let Err(e) = tx.send("Hello") {
            eprintln!("receiver dropped: {e}");
        }
    });

    let task2 tokio::spawn(async move {
        match rx.await {
            Ok(v) => println!("received: {v}") ;
            Err(e) => println!("transmitter dropped: {e}");
        }
    });

    task1.await.expect("task1 was terminated");
    task2.await.expect("task2 was terminated");
}
