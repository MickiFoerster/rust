use std::{thread, time};

use crossbeam::channel;

fn main() {
    let (ch_in, ch_out) = channel::bounded(0);

    let thread1 = thread::spawn(move || loop {
        ch_in.send("message sent by thread1").expect("send failed");
        //thread::sleep(time::Duration::from_millis(1000));
        //occ
    });
    let thread2 = thread::spawn(move || loop {
        let received = ch_out.recv().expect("could not receive message");
        println!("thread2 received: '{}'", received);
        thread::sleep(time::Duration::from_millis(1000));
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
