use std::str::FromStr;
use std::path::Path;

use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use exif::{In, Tag};

pub fn get_creation_time(path: &Path) -> Option<DateTime<Utc>> {
        let file = std::fs::File::open(path).ok()?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).ok()?;
        let f = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
        let date_str = format!("{}Z", f.display_value());
        DateTime::from_str(&date_str).ok()
}

pub fn recursive_search(path: &Path) {
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| { 
            //println!("{:?}", e); 
            e.ok()
        })
    {
        if let Some(create_date) = get_creation_time(entry.path()) {
            println!("{}    {}", entry.path().display(), create_date);
        }
    }
}
