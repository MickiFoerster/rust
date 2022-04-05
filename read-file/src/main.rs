use std::fs;
use std::io::*;

fn main() -> Result<()> {
    let filename = "/etc/hostname";
    println!("Reading file {}", filename);

    let content = fs::read_to_string(filename).expect("error: could not read file");

    print!("Content of file {}:\n{}", filename, content);

    let mut f = std::fs::File::open(filename)?;
    let mut buffer = [0; 128];
    let n = match f.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("error: {}", e);
            return Err(e);
        }
    };
    for i in 0..n {
        print!("{:?}  ", buffer[i]);
    }
    println!();

    Ok(())
}
