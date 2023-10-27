use anyhow::Result;
use rustc_span::SourceFile;
use std::io::Read;

fn main() -> Result<()> {
    let mut f = std::fs::File::open("input.rs")?;
    let mut src = String::new();
    f.read_to_string(&mut src)?;

    println!("{}", src);

    let file_name = rustc_span::FileName::Custom("main.rs".into());
    let src_file = SourceFile::new(
        file_name,
        src,
        rustc_span::BytePos(0),
        rustc_span::SourceFileHashAlgorithm::Sha256,
    );

    Ok(())
}
