pub async fn hash_computation_manager(
    mut path_rx: tokio::sync::mpsc::Receiver<std::path::PathBuf>,
    hash_map: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, String>>,
    >,
    done: tokio::sync::oneshot::Sender<()>,
) {
    let (hash_tx, mut hash_rx) = tokio::sync::mpsc::channel(256);

    // Task for spawning all sub-tasks for computing all hashes
    tokio::spawn(async move {
        while let Some(path) = path_rx.recv().await {
            let hash_tx = hash_tx.clone();
            tokio::spawn(async move {
                compute_hash(path, hash_tx).await;
            });
        }
    });

    // wait for the results
    while let Some(result) = hash_rx.recv().await {
        let (path, hash) = result;
        {
            let mut hash_map = match hash_map.lock() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("locking of hash map failed for file: {e}");
                    continue;
                }
            };
            hash_map.insert(path, hash);
            eprint!("{:6} hashes computed\r", hash_map.len());
        }
    }

    // signal that we are done
    if done.send(()).is_err() {
        eprintln!("could not send oneshot after hashing");
    }
}

async fn compute_hash(
    path: std::path::PathBuf,
    hash_tx: tokio::sync::mpsc::Sender<(std::path::PathBuf, String)>,
) {
    match tokio::fs::read(path.clone()).await {
        Ok(data) => {
            let hash = sha256::digest(data);
            if hash_tx.send((path.clone(), hash)).await.is_err() {
                eprintln!("could not send hash of file {:?}", path);
            }
        }
        Err(e) => {
            eprintln!("reading file {:?} failed: {}", path, e);
        }
    }
}
