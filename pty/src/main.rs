extern crate pty;
extern crate libc;

use std::sync::mpsc;
use std::io::{Read,Write};
use std::env;
use pty::fork::*;
use std::str;
use std::thread;

fn main() {
    let cmds = vec![
        "hostname", 
        "cat /proc/cpuinfo",
        "cat /proc/meminfo",
    ];
    let mut i = 0;
    let args: Vec<String> = env::args().collect();

    let host = match args.len() {
        2 => &args[1],
        _ => {
            eprintln!("syntax error: missing remote host to which we connect");
            eprintln!("usage: {} <remote host>", args[0]);
            std::process::exit(1);
        }
    };

    println!("try to connect to host {}", host);

    let fork = Fork::from_ptmx().unwrap();
    if let Some(mut master) = fork.is_parent().ok() {
        let (reader_tx, writer_rx) = mpsc::channel();
        let (writer_tx, reader_rx) = mpsc::channel::<String>();

        let reader = thread::spawn(move || {
            loop {
                let prompt = reader_rx.recv().unwrap();
                println!("reader received {}", prompt);
                if prompt == "" { // found end signal?
                    break;
                }
                loop {
                    let mut buffer = [0; 4096];
                    match master.read(&mut buffer[..]) {
                        Ok(_) => {
                            let output = str::from_utf8(&buffer).unwrap();
                            print!("{}", output);
                            let pattern = format!("\n{}", prompt);
                            if let Some(_) = output.find(&pattern) {
                                println!("reader found {}", prompt);
                                break;
                            }
                        },
                        Err(e)     => { 
                            println!("read error: {}", e);
                            break;
                        },
                    }
                }
                reader_tx.send(1).unwrap();
            }
            reader_tx.send(0).unwrap();
        });

        let writer = thread::spawn(move || {
            loop {
                if i >= cmds.len() {
                    println!("writer done");
                    writer_tx.send("".to_string()).unwrap();
                    break;
                }
                let cmd = cmds[i];
                i += 1;
                println!("next command: {}", cmd);
                let prompt = format!("ASDF{}", i);
                master.write(format!("PS1={}\n", prompt).as_bytes()).unwrap();
                match master.write(format!("{}\n", cmd).as_bytes()) {
                    Ok(_) => {
                        writer_tx.send(prompt).unwrap();
                    },
                    Err(e) => panic!("error: could not write: {}", e),
                }

                let token = writer_rx.recv().unwrap();
                println!("writer received {}", token);
            }

            // wait for reader to finish
            println!("writer waits for END token ...");
            let token = writer_rx.recv().unwrap();
            if token == 0 {
                println!("writer received END token");
            } else {
                panic!("error: unexpected token");
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();
    }
    else {
        std::process::Command::new("ssh").args(&[host]).status().expect("ssh command failed");
    }
}
