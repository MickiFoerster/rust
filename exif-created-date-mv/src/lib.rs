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
    pub create_date: Option<DateTime<Utc>>,
}

fn get_media_file(path: &Path) -> Option<MediaFile> {
    let file = std::fs::File::open(path).ok()?;
    let mut media_file = MediaFile {
        name: path.file_name()?.to_string_lossy().into(),
        path: PathBuf::from(path),
        len: file.metadata().ok()?.len(),
        create_date: None,
    };
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;
    match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        Some(f) => {
            let date_str = format!("{}Z", f.display_value());
            media_file.create_date = DateTime::from_str(&date_str).ok();

            Some(media_file)
        }
        None => Some(media_file),
    }
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
        match entry.metadata() {
            Ok(d) => {
                if d.is_dir() {
                    continue;
                }
            }
            Err(err) => {
                eprintln!("could not extract metadata: {}", err);
                continue;
            }
        }
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
        match f.create_date {
            Some(d) => {
                let path = get_dest_dir_path(dest_dir, &d);
                let new_filename = get_dest_filename(&f.name, d);
                let dest_file_path = path.join(new_filename);
                println!("path: {}", dest_file_path.display());

                std::fs::create_dir_all(&path)?;
                let expected_len = std::fs::copy(&f.path, &dest_file_path)?;
                if f.len != expected_len {
                    eprintln!(
                        "error: number of copied bytes ({}) was expected to be {}",
                        f.len, expected_len
                    );
                }
            }
            None => {
                // create folder under dest_dir with name of parent folder of source file
                // copy file to the created folder
                let base_name = match f.path.parent() {
                    Some(parent) => match parent.file_name(){
                        Some(p) => String::from(p.to_string_lossy()),
                        None => continue,
                    },
                    None => continue,
                };

                let path = dest_dir.join(base_name);
                let dest_file_path = path.join(&f.name);
                println!("path: {}", dest_file_path.display());

                std::fs::create_dir_all(&path)?;
                let expected_len = std::fs::copy(&f.path, &dest_file_path)?;
                if f.len != expected_len {
                    eprintln!(
                        "error: number of copied bytes ({}) was expected to be {}",
                        f.len, expected_len
                    );
                }
            }
        }
    }

    Ok(())
}

fn get_dest_filename(old_filename: &str, d: DateTime<Utc>) -> PathBuf {
    let new_filename = format!(
        "{:04}-{:02}-{:02}_{:02}_{:02}-{:02}",
        d.year(),
        d.month(),
        d.day(),
        d.hour(),
        d.minute(),
        old_filename
    );

    PathBuf::from(new_filename)
}

fn get_dest_dir_path(base_dir: &Path, date: &DateTime<Utc>) -> PathBuf {
    base_dir
        .join(format!("{:04}", date.year()))
        .join(format!("{:02}", date.month()))
    //.join(format!("{:02}", date.day()))
}
