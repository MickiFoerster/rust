use std::str::FromStr;

use chrono::{DateTime, Utc};
use exif::{In, Tag};

pub fn get_creation_time(path: &str) -> Option<DateTime<Utc>> {
        let file = std::fs::File::open(path).ok()?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).ok()?;
        let f = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
        let date_str = format!("{}Z", f.display_value());
        DateTime::from_str(&date_str).ok()
}
