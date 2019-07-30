use std::fs;
use std::path::Path;

use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, Utc, NaiveDateTime};

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

    fs::metadata(target)
        .map_err(|e| e.to_string())
        .and_then(|m| m.modified().map_err(|e| e.to_string()))
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| e.to_string()))
        .and_then(|n| Ok(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(n.as_secs() as i64, 0), Utc)))

    /*
    match fs::metadata(target) {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(time) => {
                    match time.duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(n) => Ok(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(n.as_secs() as i64, 0), Utc)),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
    */
}

pub fn parse_http_date(s: String) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    //let date_str = format!("{} GMT", s.as_str()[..s.len()-6]);
    //DateTime::<FixedOffset>::parse_from_rfc2822(date_str.as_str()).map(|dt| dt.with_timezone(&Utc))
    //DateTime::<FixedOffset>::parse_from_rfc2822(s.as_str()).map(|dt| dt.with_timezone(&Utc))
   
    let str_rfc2822 = format!("{} +0000", s.as_str()[..s.len()-4]);
    DateTime::<FixedOffset>::parse_from_rfc2822(str_rfc2822.as_str()).map(|dt| dt.with_timezone(&Utc))
}

#[test]
fn test_parse_http_date() {
    //let s = "Sat, 29 Oct 1994 19:43:31 GMT";
    let s = "Sat, 29 Oct 1994 19:43:31 +0000";
    //let s = "1996-12-19T16:39:57-08:00";
    let dt = parse_http_date(format!("{} GMT", s));

    assert_eq!(dt.map(|d| d.to_rfc2822()), Ok(s.to_string()));
}
