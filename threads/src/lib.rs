use std::sync::mpsc;
use std::thread;

pub fn spawn(tx: mpsc::Sender<Vec<u8>>) -> mpsc::Sender<()> {
    let (ctrl_tx, ctrl_rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            tx.send(b"Here is the thread".to_vec()).unwrap();
            if let Ok(()) = ctrl_rx.try_recv() {
                break;
            }
        }
        println!("thread finished");
    });

    ctrl_tx
}
