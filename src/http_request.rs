use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::method;
use crate::util;

const CR: u8 = 13;
const LF: u8 = 10;
const SP: u8 = 32;
const HT: u8 = 9;
const DQ: u8 = 34;

fn is_char(u: u8) -> bool {
    return u <= 127;
}

fn is_ctl(u: u8) -> bool {
    return u == 127 || u <= 31;
}

fn is_tspecial(u: u8) -> bool {
    let ts = vec![
        40,  // "("
        41,  // ")"
        60,  // "<"
        62,  // ">"
        64,  // "@"
        44,  // ","
        59,  // ";"
        58,  // ":"
        92,  // "\\"
        DQ,  // "\""
        47,  // "/"
        91,  // "["
        93,  // "]"
        63,  // "?"
        61,  // "="
        123, // "{"
        125, // "}"
        SP, HT,
    ];

    for i in ts {
        if i == u {
            return true;
        }
    }
    return false;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Version {
    V0_9,
    V1_0,
    V1_1,
}

impl Version {
    pub fn to_string(&self) -> String {
        match self {
            Version::V0_9 => "0.9".to_string(), // The server would never generate this.
            Version::V1_0 => "1.0".to_string(),
            Version::V1_1 => "1.1".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    //bytes: Cursor<Vec<u8>>,
    bytes: Vec<u8>,

    method: method::Method,
    pub uri: String,
    pub version: Version,
    header: HashMap<String,String>,

    entity_body: String,

    idx: usize,
    space_count: u32,
}

pub fn new() -> Request {
    Request {
        //bytes: Cursor::new(Vec::new()),
        bytes: Vec::new(),
        method: method::GET,
        uri: String::new(),
        version: Version::V0_9,
        header: HashMap::new(),

        entity_body: String::new(),

        idx: 0,
        space_count: 0,
    }
}

impl Request {
    fn skip_space(&mut self) {
        let mut length = 0;
        while self.idx + length < self.bytes.len() {
            match self.bytes[self.idx + length] {
                //SP | CR | LF => {
                SP => {
                    length += 1;
                }
                _ => {
                    break;
                }
            }
        }
        self.idx += length;
    }

    fn next_word(&mut self) -> Option<&str> {
        self.skip_space();
        let mut length = 1;
        if self.idx + length >= self.bytes.len() {
            return None;
        }
        while self.idx + length < self.bytes.len() {
            match self.bytes[self.idx + length] {
                SP => {
                    break;
                }
                CR => {
                    if length == 1 {
                        length += 1;
                    } else {
                        break;
                    }
                }
                LF => {
                    length += 1;
                    break;
                }
                _ => {
                    length += 1;
                }
            }
        }

        match std::str::from_utf8(&self.bytes[self.idx..self.idx + length]) {
            Ok(s) => {
                self.idx += length;
                Some(s)
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    fn next_token(&mut self) -> Option<&str> {
        let mut length = 0;
        while self.idx + length < self.bytes.len() {
            // token = 1*<any CHAR except CTLs or tspecials>
            let u = self.bytes[self.idx + length];
            if is_char(u) && !is_ctl(u) && !is_tspecial(u) {
                length += 1;
            } else {
                break;
            }
        }

        match std::str::from_utf8(&self.bytes[self.idx..self.idx + length]) {
            Ok(s) => {
                self.idx += length;
                Some(s)
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    fn try_crlf(&self) -> Option<&str> {
        if self.idx + 1 < self.bytes.len() {
            if self.bytes[self.idx] == CR && self.bytes[self.idx + 1] == LF {
                return Some("\r\n");
            }
        }

        None
    }

    fn try_lws(&self) -> Option<&str> {
        let u = self.bytes[self.idx];
        if u == SP || u == HT {
            let length = 1;
            return match std::str::from_utf8(&self.bytes[self.idx - length..self.idx]) {
                Ok(s) => Some(s),
                Err(_) => None,
            };
        }
        if self.idx + 2 < self.bytes.len() {
            let v = self.bytes[self.idx + 1];
            let w = self.bytes[self.idx + 2];
            if v == LF && (w == SP || w == HT) {
                let length = 3;
                return match std::str::from_utf8(&self.bytes[self.idx - length..self.idx]) {
                    Ok(s) => Some(s),
                    Err(_) => None,
                };
            }
        }
        return None;
    }

    fn parse_header_field_value(&mut self) -> Option<&str> {
        // field-value    = *( field-content | LWS )
        //
        // LWS            = [CRLF] 1*( SP | HT )
        //
        // field-content  = <the OCTETs making up the field-value
        //                  and consisting of either *TEXT or combinations
        //                  of token, tspecials, and quoted-string>
        //
        // TEXT           = <any OCTET except CTLs,
        //                  but including LWS>
        //
        // quoted-string  = ( <"> *(qdtext) <"> )
        //
        // qdtext         = <any CHAR except <"> and CTLs,
        //                  but including LWS>

        // TODO combinations of token, tspecials, and quoted-string

        // *TEXT
        let mut length = 0;
        while self.idx + length < self.bytes.len() {
            let u = self.bytes[self.idx + length];
            if u == CR || u == SP || u == HT {
                match self.try_lws() {
                    Some(s) => {
                        length += s.len();
                    }
                    None => {
                        length -= 1;
                        break;
                    }
                }
            } else if is_ctl(u) {
                length -= 1;
                break;
            } else {
                length += 1;
            }
        }

        if length > 0 {
            self.idx += length;
            return match std::str::from_utf8(&self.bytes[self.idx - length..self.idx]) {
                Ok(s) => Some(s),
                Err(_) => None,
            };
        }

        None
    }

    // TODO Divide headers into General-Header, Request-Header, Entity-Header.
    fn parse_header_entry(&mut self) -> Result<(), String> {
        // HTTP-header = field-name ":" [ field-value ] CRLF

        let name = match self.next_token() {
            Some(s) => s.to_string(),
            None => {
                return Err("Error: filed name of request header.".to_string());
            }
        };

        self.idx += 1; // Expect ':'.

        let value = match self.parse_header_field_value() {
            Some(s) => s.to_string(),
            None => {
                return Err("Error: filed value of request header.".to_string());
            }
        };

        self.idx += 2; // Expect "CR LF".
        self.header.insert(name.clone(), value.clone());

        Ok(())
    }

    pub fn parse(&mut self, content: &mut Vec<u8>) -> Result<(), String> {
        if self.bytes.len() > 0 {
            return Ok(()); // already done.
        }

        self.bytes.append(content);

        if self.bytes.len() < 4 {
            return Err("The content is too short.".to_string());
        }

        match self.next_word() {
            Some("GET") => {
                self.method = method::GET;
            }
            Some("HEAD") => {
                self.method = method::HEAD;
            }
            Some("POST") => {
                self.method = method::POST;
            }
            m => {
                return Err(format!("The content has an unknown method: {:?}", m));
            }
        }

        match self.next_word() {
            Some(s) => {
                self.uri = s.to_string();
            }
            None => {
                return Err("illegal state".to_string());
            }
        }

        match self.next_word() {
            Some("HTTP/1.0") => {
                self.version = Version::V1_0;
            }
            Some("HTTP/1.1") => {
                self.version = Version::V1_1;
            }
            Some("\r\n") => {
                self.version = Version::V0_9;
                return Ok(());
            }
            a => {
                return Err(format!("invalid token: {:?}", a));
            }
        }

        self.idx += 2; // CR LF

        // Header Fields
        while let Ok(()) = self.parse_header_entry() {
            let crlf = self.try_crlf();
            if crlf != None {
                // the end of header fields.
                self.idx += 2;
                break;
            }
        }

        // Entity-Body
        if self.idx < self.bytes.len() {
            for b in self.bytes[self.idx..].iter() {
                self.entity_body.push(char::from(*b));
            }
        }

        return Ok(());
    }

    pub fn from(&self) -> Option<&String> {
        self.header.get("From")
    }

    pub fn if_modified_since(&self) -> Option<DateTime<Utc>> {
        match self.header.get("If-Modified-Since") {
            Some(s) => match util::parse_http_date(s) {
                Ok(dt) => Some(dt),
                Err(_) => None,
            }
            None => None,
        }
    }

    pub fn referer(&self) -> Option<&String> {
        self.header.get("Referer")
    }

    pub fn user_agent(&self) -> Option<&String> {
        self.header.get("User-Agent")
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
