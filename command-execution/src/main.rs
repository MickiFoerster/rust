use std::process::{Command, ExitStatus};

fn main() {
    let mut cmd = Command::new("exiftool");
    match cmd.output() {
        Ok(v) => v,
        Err(err) => {
            eprintln!(
                "{:?} probably binary is not installed or not in PATH: {err}",
                cmd
            );
            std::process::exit(1);
        }
    };

    let file = "/home/user/Pictures/test/14340dca33f9efa846b476924129ac4316d7e658768f5552d83c200a24278d41.mts";
    let output = match cmd.arg(file).output() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{:?} error: {err}", cmd);
            std::process::exit(1);
        }
    };

    if cmd.status().expect("could not get exit status").success() {
        let mut counter = 0;
        let stdout_output =
            std::str::from_utf8(&output.stdout).expect("UTF-8 encoded string expected");
        for line in stdout_output.lines() {
            counter += 1;
            let pos = match line.find(':') {
                Some(p) => p,
                None => continue,
            };
            let key = line[0..pos].trim();
            let value = line[pos + 1..].trim();
            if key == "Date/Time Original" {
                println!("{line}");
                println!("{}", value);
            }
        }
        println!("{counter} lines printed");
    } else {
        println!("command failed");
    }
}
