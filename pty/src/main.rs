extern crate pty;
extern crate libc;

use std::io::{Read,Write};
use std::env;
use pty::fork::*;
use std::str;
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fork = Fork::from_ptmx().unwrap();

    let host = match args.len() {
        2 => &args[1],
        _ => {
            eprintln!("syntax error: missing remote host to which we connect");
            eprintln!("usage: {} <remote host>", args[0]);
            std::process::exit(1);
        }
    };

    println!("try to connect to host {}", host);

    if let Some(mut master) = fork.is_parent().ok() {
        let reader = thread::spawn(move || {
            loop {
                let mut buffer = [0; 4096];
                match master.read(&mut buffer[..]) {
                    Ok(n) => {
                        let output = str::from_utf8(&buffer).unwrap();
                        print!("{}", output)
                    },
                    Err(e)     => panic!("read error: {}", e),
                }
            }
        });

        let writer = thread::spawn(move || {
            loop {
                match master.write("hostname\n".as_bytes()) {
                    Ok(_) => {},
                    Err(e) => panic!("error: could not write: {}", e),
                }
                thread::sleep(time::Duration::from_millis(1000));
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();
    }
    else {
        std::process::Command::new("ssh").args(&[host]).status().expect("ssh command failed");
    }
}
