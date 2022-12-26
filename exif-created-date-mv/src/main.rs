use exif_created_date_mv::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    recursive_search(Path::new("/home/micki/Pictures"));

    Ok(())
}
