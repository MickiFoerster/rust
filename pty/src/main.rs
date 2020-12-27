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
                // Now, read until prompt provided by writer is found.
                let mut output = String::new();
                loop {
                    let mut read_output: String;
                    let mut phase = 1;
                    loop {
                        let mut buffer = [0; 4096];
                        match master.read(&mut buffer[..]) {
                            Ok(n) => {
                                read_output = str::from_utf8(&buffer[0..n]).unwrap().to_string();

// Solution should:
// - read line by line 
// - first phase output should not go into outer variable
// - second phase pattern found means command output starts now
// - when third pattern is found command output has finished

                                print!("phase {}: read_output: {}", phase, read_output);
                                match phase {
                                    1 => {
                                        let pattern = format!("\nPS1={}", prompt);
                                        if let Some(i) = read_output.find(&pattern) {
                                            println!("phase {}: reader found {}", phase, pattern);
                                            phase = 2;
                                            let len = pattern.chars().count();
                                            print!("phase {}: add {}", phase, &read_output[0..i+len]);
                                            print!("phase {}: new buffer {}", phase, &read_output[i+len..]);
                                            output.push_str(&read_output[0..i+len]);
                                            read_output = read_output[i+len..].to_string();
                                            std::process::exit(1);
                                        }
                                    },
                                    2 => {
                                        let pattern = format!("\n{}", prompt);
                                        print!("phase {}: reader looks for '{}'", phase, pattern);
                                        if let Some(_) = read_output.find(&pattern) {
                                            println!("phase {}: reader found {}", phase, pattern);
                                            phase = 3;
                                        }
                                    },
                                    3 => {
                                        let pattern = format!("\n{}", prompt);
                                        print!("phase {}: reader looks for '{}'", phase, pattern);
                                        if let Some(_) = read_output.find(&pattern) {
                                            println!("phase {}: reader found {}", phase, pattern);
                                            break;
                                        }
                                    },
                                    _ => panic!("unexpected case: phase == {}\n", phase),
                                }
                            },
                            Err(e)     => { 
                                panic!("read error: {}", e); 
                            },
                        }
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
