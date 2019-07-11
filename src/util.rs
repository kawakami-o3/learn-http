use std::path::Path;

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