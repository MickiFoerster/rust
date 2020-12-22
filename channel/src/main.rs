use std::sync::mpsc;
use std::thread;
use std::time;

fn main() {
    let (thread1_tx, thread1_rx) = mpsc::channel();
    let (thread2_tx, thread2_rx) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        loop {
            let ping = String::from("ping");
            thread1_tx.send(ping).unwrap();

            let received = thread2_rx.recv().unwrap();
            println!("thread1 received: {}", received);
        }
    });
    let thread2 = thread::spawn(move || {
        loop {
            let received = thread1_rx.recv().unwrap();
            println!("thread2 received: {}", received);

            thread::sleep(time::Duration::from_millis(1000));

            let pong = String::from("pong");
            thread2_tx.send(pong).unwrap();
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
