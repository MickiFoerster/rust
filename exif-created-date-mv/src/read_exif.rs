use std::path::Path;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use exif::{In, Tag};

pub fn get_created_date(path: &Path) -> Option<DateTime<Utc>> {
    let file = std::fs::File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("error: could not read exif data: {}", err);
            return None;
        }
    };

    let field = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
    let date_str = format!("{}Z", field.display_value());
    DateTime::from_str(&date_str).ok()
}
