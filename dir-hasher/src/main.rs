use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let hash_set = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0 as u64));

    for entry in walkdir::WalkDir::new(args.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = std::fs::metadata(entry.path()).unwrap();
        if metadata.is_dir() {
            continue;
        }

        {
            let mut counter = counter.lock().expect("lock failed");
            *counter += 1;
        }

        let hash_set = hash_set.clone();
        let counter = counter.clone();

        tokio::spawn(async move {
            let file_content = match tokio::fs::read(entry.path()).await {
                Ok(v) => v,
                Err(_) => {
                    {
                        let mut counter = counter.lock().expect("lock failed");
                        *counter -= 1;
                    }
                    return;
                }
            };

            let hash = sha256::digest(file_content);

            hash_set
                .lock()
                .expect("locking hashmap failed")
                .insert(std::path::PathBuf::from(entry.path()), hash);

            //println!("hash added for {}", entry.path().display());

            {
                let mut counter = counter.lock().expect("lock failed");
                *counter -= 1;
            }
        });
    }

    loop {
        {
            let counter = counter.lock().expect("lock failed");
            if *counter == 0 {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let hash_set = hash_set.lock().unwrap();
    for (p, h) in &*hash_set {
        println!("{:120} -> {}", p.display(), h);
    }
}
