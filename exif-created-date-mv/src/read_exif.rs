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
            // Use fallback exiftool
            match exiftool(path, "Date/Time Original") {
                Some(v) if v.len() < 10 => {
                    eprintln!("error: invalid date format in exif data: {}", v);
                    return None;
                }
                Some(v) => {
                    let date = v[0..10].replace(':', "-");
                    let substr = &v[11..];
                    let t = match substr.find('+') {
                        Some(pos) => &substr[0..pos],
                        None => return None,
                    };
                    let date_str = format!("{} {}Z", date, t);
                    let date = DateTime::from_str(&date_str).ok();
                    return date;
                }
                None => {
                    eprintln!("error: could not read exif data: {}", err);
                    return None;
                }
            }
        }
    };

    let field = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
    let date_str = format!("{}Z", field.display_value());
    DateTime::from_str(&date_str).ok()
}

fn exiftool(path: &Path, key_pattern: &str) -> Option<String> {
    let mut cmd = std::process::Command::new("exiftool");
    if let Err(err) = cmd.output() {
        eprintln!("{:?} error: {err}", cmd);
        return None;
    }

    println!("exiftool was found. Provide path '{}' to exiftool now.", path.display());
    let output = match cmd
        .arg(path)
        .output()
    {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{:?} error: {err}", cmd);
            return None;
        }
    };

    let output = std::str::from_utf8(&output.stdout).ok()?;
    for line in output.lines() {
        let pos = match line.find(':') {
            Some(p) => p,
            None => continue,
        };
        let key = line[0..pos].trim();
        let value = line[pos + 1..].trim();
        if key.eq(key_pattern) {
            return Some(String::from(value));
        }
    }

    None
}
