use std::io::{Read, Write};

const BUFFER_SIZE: usize = 4 * 1024;

fn main() {
    let mut total_bytes = 0;
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let n = match std::io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                eprintln!("read error: {err}");
                break;
            }
        };
        total_bytes += n;
        std::io::stdout().write_all(&buffer[..n]).unwrap();
    }
    eprintln!("{total_bytes} bytes read");
}
