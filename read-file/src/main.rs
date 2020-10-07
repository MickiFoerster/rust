//use std::env;
use std::fs;

fn main() {
    let filename = "/etc/hostname";
    println!("Reading file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("error: could not read file");

    print!("Content of file {}:\n{}", filename, content);
}
