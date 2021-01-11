use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("cat")
            .args(&["/proc/cpuinfo"])
            .output()
            .expect("failed to execute process");

    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("command stdout: {}", s);

    let output = Command::new("ls")
            .args(&["-1", "/proc"])
            .output()
            .expect("failed to execute 'ls'");
    let output_as_string = str::from_utf8(&output.stdout).unwrap();
    for w in output_as_string.split("\n") {
        println!("ls: {}", w);
    }

    // unknown command
    let output = Command::new("unknowncommand")
            .args(&["--verbose"])
            .output()
            .expect("When command fails we panic!");
}
