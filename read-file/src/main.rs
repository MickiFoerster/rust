use std::fs;
use std::io::*;

fn main() -> Result<()> {
    let filename = "/etc/hostname";
    println!("Reading file {}", filename);

    // Method 1
    let content = fs::read_to_string(filename).expect("error: could not read file");
    print!("Content of file {}:\n{}", filename, content);

    // Method 2
    let mut f = std::fs::File::open(filename)?;
    let mut buffer = [0; 1];
    loop {
        let n = match f.read(&mut buffer) {
            Ok(0) => {
                println!("0 bytes read. So, we assume EOF has been reached.");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("error: {}", e);
                return Err(e);
            }
        };
        /*
        for i in 0..n {
            print!("0x{:02x?}  ", buffer[i]);
        }
        */
        print!("{}", String::from_utf8(buffer[0..n].to_vec()).expect("Cannot convert bytes to string"));
    }
    drop(f);

    Ok(())
}
