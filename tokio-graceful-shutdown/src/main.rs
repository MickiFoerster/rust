use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel();

    // tasks get a cancellation token. With this we can signal to terminate
    // from the main task.
    let token = tokio_util::sync::CancellationToken::new();

    let cloned_token_task1 = token.clone();
    let shutdown_send_task1 = shutdown_send.clone();
    tokio::spawn(async move {
        println!("start task1 which gets signalled when it should finish its work");
        tokio::select! {
            _ = cloned_token_task1.cancelled() => {
                println!("received signal to cancel work");
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
                println!("task1 finished its hard work of sleeping for 10 seconds");
            }
        }
        println!("finish task1");
        shutdown_send_task1
            .send("task1 finished successful")
            .expect("could not send");
    });

    let cloned_token_task2 = token.clone();
    let shutdown_send_task2 = shutdown_send.clone();
    tokio::spawn(async move {
        println!("start task2 which gets signalled when it should finish its work");
        tokio::select! {
            _ = cloned_token_task2.cancelled() => {
                println!("received signal to cancel work");
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                println!("task2 finished its hard work of sleeping for 5 seconds");
            }
        }
        println!("finish task2");
        shutdown_send_task2
            .send("task2 finished successful")
            .expect("could not send");
    });

    //
    // application uses shutdown_send in case a shutdown was issued from inside
    // the application

    // drop sending end of channel of the main task. Otherwise, the channel won't be closed after
    // all sub-tasks have sent their end.
    drop(shutdown_send);

    let mut finished_tasks = 0;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("graceful shutdown by CTRL-C");
                println!("signal tasks to finish their work with help of cancellation token ...");
                token.cancel();
                println!("token canceled");
            },
            msg = shutdown_recv.recv() => {
                if let Some(msg) = msg {
                    println!("received shutdown acknowledgement from task: {msg}");

                    finished_tasks += 1;
                    if finished_tasks == 2 { break }
                }
            },
            else => { break },
        }
    }

    // send shutdown signal to application and wait
    println!("end of main task");
}
