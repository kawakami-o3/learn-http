use std::fs;
use std::path::Path;

use std::time::SystemTime;
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn canonicalize(s: &str) -> Option<String> {
    let mut v: Vec<&str> = Vec::new();
    for i in s.split("/") {
        match i {
            "" => {
                if v.len() == 0 {
                    v.push("");
                }
            }
            ".." => {
                v.pop();
            }
            a => {
                v.push(a);
            }
        }
    }

    v.retain(|&x| x != ".");
    if v.len() == 0 {
        return None;
    }
    if v[0] != "" {
        return None;
    }

    Some(v.join("/"))
}

pub fn extension(target: &String) -> Option<&str> {
    match Path::new(target.as_str()).extension() {
        Some(s) => s.to_str(),
        _ => None,
    }
}

pub fn modified(target: &String) -> Result<DateTime<Utc>, String> {
    match fs::metadata(target) {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(time) => {
                    match time.duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(n) => Ok(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(n.as_secs() as i64, 0), Utc)),
                        Err(e) => Err(format!("{}", e)),
                    }
                }
                Err(e) => Err(format!("{}", e)),
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}
