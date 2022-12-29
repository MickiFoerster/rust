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

fn recursive_search(path: &Path) -> Vec<MediaFile> {
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

pub fn copy_files_to_dest_dir(source_dir: &Path, dest_dir: &Path) -> Result<usize, std::io::Error> {
    let files = recursive_search(Path::new(source_dir));
    let n = files.len();
    println!(
        "Recursive search has been found {} files. Now copying starts ...",
        n
    );

    for f in files.iter() {
        let path: PathBuf;
        let dest_file_path: PathBuf;

        match f.create_date {
            Some(d) => {
                path = get_dest_dir_path(dest_dir, &d);
                let new_filename = get_dest_filename(&f.name, d);
                dest_file_path = path.join(new_filename);
            }
            None => {
                // create folder under dest_dir with name of parent folder of source file
                let prefix = f
                    .path
                    .strip_prefix(source_dir)
                    .expect("cannot extract path prefix")
                    .parent()
                    .expect("cannot get parent folder");

                path = dest_dir.join(prefix);
                dest_file_path = path.join(&f.name);
            }
        }

        if let Err(err) = copy_media_file(&path, &f.path, &dest_file_path, f.len) {
            eprintln!(
                "error: file {} could not be copied: {}",
                f.path.display(),
                err
            );
            continue;
        }
    }

    Ok(n)
}

fn copy_media_file(
    path: &Path,
    file_source_path: &Path,
    file_dest_path: &Path,
    file_len: u64,
) -> Result<(), std::io::Error> {
    println!("path: {}", file_dest_path.display());
    std::fs::create_dir_all(path)?;
    let expected_len = std::fs::copy(&file_source_path, file_dest_path)?;
    if file_len != expected_len {
        eprintln!(
            "error: number of copied bytes ({}) was expected to be {}",
            file_len, expected_len
        );
        return Err(std::io::Error::last_os_error());
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
