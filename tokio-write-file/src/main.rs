use tokio::io::AsyncWriteExt;
//use tokio::fs::File;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut file = tokio::fs::File::create("foo.txt").await?;

    // Writes some bytes but not necessarily all
    let n = file.write(b"some bytes").await?;
    println!("wrote {n} bytes to file");

    Ok(())
}

async fn write_all() -> tokio::io::Result<()> {
    let mut file = tokio::fs::File::create("foo.txt").await?;

    file.write_all(b"some bytes").await?;

    Ok(())
}

async fn reader_to_writer_copy() -> tokio::io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = tokio::fs::File::create("foo.txt").await?;

    tokio::io::copy(&mut reader, &mut file).await?;

    Ok(())
}
