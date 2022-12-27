use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, Datelike, Timelike, Utc};
use exif::{In, Tag};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub name: String,
    pub path: PathBuf,
    pub len: u64,
    pub create_date: DateTime<Utc>,
}

fn get_media_file(path: &Path) -> Option<MediaFile> {
    let file = std::fs::File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;
    let f = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
    let date_str = format!("{}Z", f.display_value());

    Some(MediaFile {
        name: path.file_name()?.to_string_lossy().into(),
        path: PathBuf::from(path),
        len: file.metadata().ok()?.len(),
        create_date: DateTime::from_str(&date_str).ok()?,
    })
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
        if let Some(file) = get_media_file(&path) {
            files.push(file);
        }
    }

    files
}

pub fn copy_files_to_dest_dir(
    files: Vec<MediaFile>,
    dest_dir: &Path,
) -> Result<(), std::io::Error> {
    for f in files.iter() {
        let path = get_dest_dir_path(dest_dir, &f.create_date);
        let new_filename = get_dest_filename(&f.name, f.create_date);
        let dest_file_path = path.join(new_filename);
        println!("path: {}", dest_file_path.display());

        std::fs::create_dir_all(&path)?;
        let expected_len = std::fs::copy(&f.path, &dest_file_path)?;
        assert_eq!(f.len, expected_len);
    }

    Ok(())
}

fn get_dest_filename(old_filename: &str, d: DateTime<Utc>) -> PathBuf {
    let new_filename = format!("{:04}-{:02}-{:02}_{:02}_{:02}-{:02}" , d.year() , d.month() , d.day() , d.hour() , d.minute(), old_filename );

    PathBuf::from(new_filename)
}

fn get_dest_dir_path(base_dir: &Path, date: &DateTime<Utc>) -> PathBuf {
    base_dir
        .join(format!("{:04}", date.year()))
        .join(format!("{:02}", date.month()))
        //.join(format!("{:02}", date.day()))
}
