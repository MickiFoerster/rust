use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

mod filesearch;
mod hashing;
// manager that gets file paths as input and spawns sub-tasks
// for calculating hashes.
// task that looks for file paths and puts file paths into channel to hash manager

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let hash_map = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
    let (tx, rx) = tokio::sync::mpsc::channel::<std::path::PathBuf>(4096);
    let (done_tx, done_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move { filesearch::file_search(args.path, tx).await });

    let hash_map_ref = hash_map.clone();
    tokio::spawn(async move { hashing::hash_computation_manager(rx, hash_map_ref, done_tx).await });

    match done_rx.await {
        Ok(()) => {
            let hash_map = hash_map.lock().unwrap();
            for (p, h) in &*hash_map {
                println!("{:120} -> {}", p.display(), h);
            }
            println!("{} hashes were computed.", hash_map.len());
        }
        Err(e) => eprintln!("error while waiting for oneshot: {e}"),
    };
}
