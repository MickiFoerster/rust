mod filesearch;
mod hashing;

type HashMap = std::collections::HashMap<std::path::PathBuf, String>;

pub async fn compute_hashes(path: std::path::PathBuf) -> Result<HashMap, String> {
    let hash_map = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
    let (tx, rx) = tokio::sync::mpsc::channel::<std::path::PathBuf>(4096);
    let (done_tx, done_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move { filesearch::file_search(path, tx).await });

    let hash_map_ref = hash_map.clone();
    tokio::spawn(async move { hashing::hash_computation_manager(rx, hash_map_ref, done_tx).await });

    match done_rx.await {
        Ok(()) => {
            let hash_map = hash_map.lock().unwrap();
            let mut new_hash_map = std::collections::HashMap::new();
            new_hash_map.clone_from(&hash_map);

            Ok(new_hash_map)
        }
        Err(e) => Err(format!("error while waiting for oneshot: {e}")),
    }
}
