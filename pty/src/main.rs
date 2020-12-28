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
    let mut current_cmd = 0;
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
                read_until_login_finished(&mut master);

                read_command_output(&mut master).expect("reading output failed");

                reader_tx.send(1).unwrap();
            }
            //reader_tx.send(0).unwrap();
        });

        let writer = thread::spawn(move || {
            loop {
                if current_cmd >= cmds.len() {
                    println!("writer done");
                    writer_tx.send("".to_string()).unwrap();
                    break;
                }

                // phase 0: wait for login
                println!("writer is waiting ...");
                let signal = writer_rx.recv().unwrap();
                println!("writer received signal from reader: {}", signal);

                let cmd = cmds[current_cmd];
                current_cmd += 1;
                println!("next command: {}", cmd);
                //writer_tx.send(cmd.to_string()).expect("could not send to reader");
                //master.write(cmd.as_bytes()).expect("could not write to master");

                //println!("writer waiting ...");
                //let token = writer_rx.recv().unwrap();
                //println!("writer should have received 2: {}", token);
                //master.write("\n".as_bytes()).expect("could not write");
                //writer_tx.send(prompt.to_string()).unwrap();
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();
    }
    else {
        std::process::Command::new("ssh").args(&[host]).status().expect("ssh command failed");
    }
}

fn read_until_login_finished(master: &mut pty::fork::Master) {
    let mut buffer = [0; 4096];
    match master.read(&mut buffer[..]) {
        Ok(n) => {
            println!("reader: read {} bytes", n);
            let read_output = str::from_utf8(&buffer[0..n]).unwrap().to_string();
            for line in read_output.lines() {
                println!("line:{}", line);
            }
        }
        Err(e)     => { 
            panic!("read error: {}", e); 
        },
    }
}

fn read_command_output(master: &mut pty::fork::Master) -> Result<String> {
    let mut output = String::new();
    println!("reader: starting ...");
    loop {
        let mut read_output: String;
        let mut phase = 0;
        loop {
            let mut buffer = [0; 4096];
            println!("reader: read from master");
            match master.read(&mut buffer[..]) {
                Ok(n) => {
                    println!("reader: read {} bytes", n);
                    read_output = str::from_utf8(&buffer[0..n]).unwrap().to_string();

                    // - process line by line 
                    // - phase 0: writer <-- signal ---- reader (login finished)
                    // - phase 1: writer --- command --> reader
                    // - phase 2: writer <-- signal  --  reader (command received)
                    // - phase 3: writer --- \n      --> reader (start reading output from command
                    //                                   waits for end of reading
                    //                                   looks for suffix "$ "
                    // - phase 4: writer <-- signal  --  reader (command execution finished)
                    // - jump to phase 1 again

                    for line in read_output.lines() {
                        match phase {
                            0 => {
                                println!("{}:line:{}", phase, line);
                            }
                            1 => { // look for PS1= input
                                //let pattern = format!("{}", prompt);
                                ////println!("phase {}: reader looks for '{}'", phase, pattern);
                                //if let Some(_) = read_output.find(&pattern) {
                                //    //println!("phase {}: reader found {}\n", phase, pattern);
                                //    reader_tx.send(1).unwrap();
                                //    phase = 2;
                                //    prompt = reader_rx.recv().unwrap();
                                //    println!("reader received command: {}", prompt);
                                //}
                            },
                            2 => { 
                            },
                            3 => {
                            },
                            4 => {
                            },
                            _ => panic!("unexpected case: phase == {}\n", phase),
                        }
                    }
                },
                Err(e)     => { 
                    panic!("read error: {}", e); 
                },
            }
        }
    }
}
