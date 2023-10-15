use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 4 * 1024;

#[tokio::main]
async fn main() {
    let mut total_bytes = 0;
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let n = match tokio::io::stdin().read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                eprintln!("read error: {err}");
                break;
            }
        };
        total_bytes += n;
        tokio::io::stdout().write_all(&buffer[..n]).await.unwrap();
    }
    eprintln!("{total_bytes} bytes read");
}
