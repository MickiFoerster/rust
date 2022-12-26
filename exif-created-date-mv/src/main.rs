use exif_created_date_mv::get_creation_time;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    for path in &["/home/micki/Pictures/IMG_20180208_073937.jpg"] {
        match get_creation_time(path) {
            Some(d) => println!("{}: {}", path, d),
            None => println!("No creation date found"),
        }
    }

    Ok(())
}
