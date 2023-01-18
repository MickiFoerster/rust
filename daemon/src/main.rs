use fork::{daemon, Fork};
use std::process::Command;

fn main() {
    match daemon(false, false) {
        Ok(Fork::Child) => {
            Command::new("sleep")
                .arg("10")
                .output()
                .expect("failed to execute process");
        }
        Ok(Fork::Parent(pid)) => {
            // This will not be printed
            println!("Daemon process with ID {} created", pid);
            std::thread::sleep(std::time::Duration::from_secs(12));
        }
        Err(err) => eprintln!("could not create daemon: {err}"),
    }
}
