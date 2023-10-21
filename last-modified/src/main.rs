use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    last_modified_hours: u64,

    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!(
        "Entries modified in the last {} hours in {:?}:",
        args.last_modified_hours,
        args.path
    );

    for entry in walkdir::WalkDir::new(args.path).follow_links(true).into_iter()
        .filter_map(|e| e.ok()) {
            let path = entry.path();

            let metadata = std::fs::metadata(&path).unwrap();
            let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

            if last_modified < args.last_modified_hours * 3600 && metadata.is_file() {
                println!(
                    "{:130} -> last modified: {:?} seconds",
                    path.display(),
                    last_modified,
                );
            }
    }
}
