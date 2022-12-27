use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, Datelike, Timelike, Utc};
use exif::{In, Tag};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub name: String,
    pub path: PathBuf,
    pub create_date: DateTime<Utc>,
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

pub fn recursive_search(path: &Path) -> Vec<MediaFile> {
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

    files
}

pub fn move_files_to_dest_dir(
    files: Vec<MediaFile>,
    dest_dir: &Path,
) -> Result<(), std::io::Error> {
    for f in files.iter() {
        let path = get_dest_dir_path(dest_dir, &f.create_date);
        println!("path: {}", path.display());
        std::fs::create_dir_all(path).expect("directory cannot be created");
    }

    Ok(())
}

fn get_dest_dir_path(base_dir: &Path, date: &DateTime<Utc>) -> PathBuf {
    base_dir
        .join(format!("{:04}", date.year()))
        .join(format!("{:02}", date.month()))
        .join(format!("{:02}", date.day()))
}
