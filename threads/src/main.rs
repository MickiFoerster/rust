use std::str;
use std::sync::mpsc;
use threads::spawn;

fn main() {
    let (tx, rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();
    let ch = spawn(tx);

    for _ in 0..100 {
        let msg = rx.recv().unwrap();
        let msg = str::from_utf8(&msg).unwrap();
        println!("received: {}", msg);
    }

    ch.send(()).unwrap();
}
