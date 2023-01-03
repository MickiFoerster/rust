use std::process::Command;

fn main() {
    let mut cmd = Command::new("exiftool");
    match cmd.output() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{:?} probably binary is not installed or not in PATH: {err}", cmd);
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
    let output = std::str::from_utf8(&output.stdout).expect("UTF-8 encoded string expected");

    for line in output.lines() {
        let pos = match line.find(':') {
            Some(p) => p,
            None => continue,
        };
        let key = line[0..pos].trim();
        let value = line[pos+1..].trim();
        if key == "Date/Time Original" {
            println!("{line}");
            println!("{}", value);
        }
    }
}
