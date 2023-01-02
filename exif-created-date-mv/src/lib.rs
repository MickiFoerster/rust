use std::io::Read;
use std::path::{Path, PathBuf};
use std::thread;

use chrono::{DateTime, Datelike, Timelike, Utc};
use crossbeam::channel;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

mod read_exif;

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub name: String,
    pub path: PathBuf,
    pub len: u64,
    pub create_date: Option<DateTime<Utc>>,
    pub hash: Option<String>,
}

fn get_media_file(path: &Path) -> Option<MediaFile> {
    let file_length = std::fs::File::open(path).ok()?.metadata().ok()?.len();

    Some(MediaFile {
        name: path.file_name()?.to_string_lossy().into(),
        path: PathBuf::from(path),
        len: file_length,
        hash: get_hash_of_file(path),
        create_date: read_exif::get_created_date(path),
    })
}
fn get_hash_of_file(path: &Path) -> Option<String> {
    let mut file = std::fs::File::open(path).ok()?;
    let len = file.metadata().ok()?.len();

    let max_size=1024*1024*1024*4; /* 4gb */
    if len > max_size {
        eprintln!(
            "File {} has size over {}mb and hash won't be computed",
            path.display(),
            max_size / (1024 * 1024)
        );
        return None;
    }

    let mut buffer = [0; 4096];
    let mut hasher = Sha256::new();

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[0..n]),
            Err(err) => {
                eprintln!("error: could not read file {}: {}", path.display(), err);
                return None;
            }
        }
    }

    Some(format!("{:X}", hasher.finalize()))
}

fn recursive_search(path: &Path, ch: channel::Sender<MediaFile>) {
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
            ch.send(file).expect("could not send file via channel");
        }
    }
}

pub fn copy_files_to_dest_dir(source_dir: &Path, dest_dir: &Path) -> Result<(), std::io::Error> {
    let (ch_in, ch_out) = channel::bounded(0);

    let p = PathBuf::from(source_dir);
    let thread = thread::spawn(move || {
        recursive_search(&p, ch_in);
    });

    for f in ch_out.iter() {
        let path: PathBuf;
        let dest_file_path: PathBuf;

        println!("Processing file {}", f.path.display());
        match f.create_date {
            Some(d) => {
                path = get_dest_dir_path(dest_dir, &d);
                let new_filename = get_dest_filename(&f, d);
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
        }
    }

    thread.join().expect("could not join worker thread");

    Ok(())
}

fn copy_media_file(
    path: &Path,
    file_source_path: &Path,
    file_dest_path: &Path,
    file_len: u64,
) -> Result<(), std::io::Error> {
    println!("path: {}", file_dest_path.display());
    std::fs::create_dir_all(path)?;
    let expected_len = std::fs::copy(file_source_path, file_dest_path)?;
    if file_len != expected_len {
        eprintln!(
            "error: number of copied bytes ({}) was expected to be {}",
            file_len, expected_len
        );
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn get_dest_filename(file: &MediaFile, d: DateTime<Utc>) -> PathBuf {
    let filename_ext = file
        .name
        .split('.')
        .last()
        .unwrap_or(&file.name[file.name.len() - 3..]);
    let new_filename = format!(
        "{:04}-{:02}-{:02}_{:02}_{:02}-{}.{}",
        d.year(),
        d.month(),
        d.day(),
        d.hour(),
        d.minute(),
        file.hash.clone().unwrap_or_default(),
        filename_ext.to_lowercase()
    );

    PathBuf::from(new_filename)
}

fn get_dest_dir_path(base_dir: &Path, date: &DateTime<Utc>) -> PathBuf {
    base_dir
        .join(format!("{:04}", date.year()))
        .join(format!("{:02}", date.month()))
    //.join(format!("{:02}", date.day()))
}
