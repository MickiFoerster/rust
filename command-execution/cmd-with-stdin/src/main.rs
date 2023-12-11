use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"Hello, world!\n")?;
    }

    let output = child.wait_with_output()?;

    //println!("output = {:?}", output);
    if output.status.success() {
        println!(
            "command execution successful: {:?}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        println!(
            "command execution failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
