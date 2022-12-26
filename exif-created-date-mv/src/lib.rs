use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use exif::{In, Tag};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct MediaFile {
    name: String,
    path: PathBuf,
    create_date: DateTime<Utc>,
}

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
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| {
            //println!("{:?}", e);
            e.ok()
        })
    {
        let path = entry.path().to_owned();
        if let Some(create_date) = get_creation_time(&path) {
            let file = MediaFile {
                name: entry.file_name().to_str().unwrap_or_default().into(),
                path,
                create_date,
            };

            files.push(file);
        }
    }

    for f in files.iter() {
        println!("{:#?}", f);
    }
}
