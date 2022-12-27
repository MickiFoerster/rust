use exif_created_date_mv::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = recursive_search(Path::new("/home/micki/Pictures"));
    for f in files.iter() {
        println!("{} : {}", f.path.display(), f.create_date);
    }

    move_files_to_dest_dir(files, Path::new("/tmp/asdf"))?;

    Ok(())
}
