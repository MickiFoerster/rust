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

async fn read_file() -> tokio::io::Result<()> {
    let mut f = tokio::fs::File::open("foo.txt").await?;

    // read up to 10 bytes
    // let mut buffer = [0; 10];
    // let n = f.read(&mut buffer).await?;
    // println!("read bytes: {:?}", &buffer[..n]);

    // read the whole file
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("read bytes: {:?}", buffer);

    Ok(())
}
