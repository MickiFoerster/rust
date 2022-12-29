use std::path::Path;

use exif_created_date_mv::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "syntax error: {} <SOURCE FOLDER> <DESTINATION FOLDER>",
            args[0]
        );
        std::process::exit(0);
    }
    let source_dir = &args[1];
    let dest_dir = &args[2];

    let files = recursive_search(Path::new(source_dir));
    copy_files_to_dest_dir(files, Path::new(source_dir), Path::new(dest_dir))?;

    Ok(())
}
