pub async fn file_search(
    path: std::path::PathBuf,
    sender: tokio::sync::mpsc::Sender<std::path::PathBuf>,
) {
    for entry in walkdir::WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        match std::fs::metadata(entry.path()) {
            Ok(v) => {
                if v.is_dir() {
                    continue;
                } else {
                    v
                }
            }
            Err(_) => continue,
        };

        sender
            .send(entry.path().to_path_buf())
            .await
            .expect("error while sending path")
    }
}
