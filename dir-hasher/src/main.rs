use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let hash_map = match dir_hasher::compute_hashes(args.path).await {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    };

    let len = hash_map.len();
    for (p, h) in hash_map {
        println!("{:120} -> {}", p.display(), h);
    }
    println!("{len} hashes were computed.");
}
