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
                let cmd = reader_rx.recv().unwrap();
                println!("reader received {}", cmd);
                if cmd == "" { // found end signal?
                    println!("reader done since received command is empty");
                    break;
                }
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
                                // - process line by line 
                                // - phase 1: look for assignment of prompt view
                                // - phase 2: look for prompt with current command
                                // - phase 3: look for prompt which represents the end of the command output

                                for line in read_output.lines() {
                                    println!("{}:line:{}", phase, line);
                                    match phase {
                                        1 => { // look for PS1= input
                                            let pattern = format!("{}", prompt);
                                            //println!("phase {}: reader looks for '{}'", phase, pattern);
                                            if let Some(_) = read_output.find(&pattern) {
                                                //println!("phase {}: reader found {}\n", phase, pattern);
                                                reader_tx.send(1).unwrap();
                                                phase = 2;
                                                prompt = reader_rx.recv().unwrap();
                                                println!("reader received command: {}", prompt);
                                            }
                                        },
                                        2 => { 
                                            let pattern = format!("{}", prompt);
                                            println!("phase {}: reader looks for '{}'", phase, pattern);
                                            if let Some(_) = read_output.find(&pattern) {
                                                println!("phase {}: reader found {}\n", phase, pattern);
                                                reader_tx.send(2).unwrap();
                                                println!("reader waits for signal that linefeed was sent ...");
                                                reader_rx.recv().unwrap();
                                                println!("reader received that linefeed was sent");
                                                phase = 3;
                                            }
                                        },
                                        3 => {
                                            let pattern = format!("{}", prompt);
                                            println!("phase {}: reader looks for '{}'", phase, pattern);
                                            if let Some(_) = read_output.find(&pattern) {
                                                println!("phase {}: reader found echo of command {}", phase, pattern);
                                                prompt = reader_rx.recv().unwrap();
                                                println!("reader received prompt before phase 4: {}", prompt);
                                                phase = 4;
                                            }
                                        },
                                        4 => {
                                            let pattern = format!("{}", prompt);
                                            println!("phase {}: reader looks for '{}'", phase, pattern);
                                            if let Some(_) = read_output.find(&pattern) {
                                                println!("end of reading output");
                                                phase = 4;
                                                break;
                                            }
                                        },
                                        _ => panic!("unexpected case: phase == {}\n", phase),
                                    }
                                }
                                //println!("loop finished");
                                //std::process::exit(1);

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
                writer_tx.send(cmd.to_string()).expect("could not send to reader");
                master.write(cmd.as_bytes()).expect("could not write to master");

                println!("writer waiting ...");
                let token = writer_rx.recv().unwrap();
                println!("writer should have received 2: {}", token);
                master.write("\n".as_bytes()).expect("could not write");
                writer_tx.send(prompt.to_string()).unwrap();

                println!("writer waiting for end of command output ...");
                let token = writer_rx.recv().unwrap();
                println!("writer should have received 3: {}", token);
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
