
//use std::io::Cursor;

//const G: u8 = 71;
//const E: u8 = 69;
//const T: u8 = 84;
const CR: u8 = 13;
const LF: u8 = 10;
const SP: u8 = 32;
const HT: u8 = 9;
const DQ: u8 = 34;

fn is_char(u: u8) -> bool {
    return u <= 127
}

fn is_ctl(u: u8) -> bool {
    return u == 127 || u <= 31
}

fn is_tspecials(u: u8) -> bool {
    let ts = vec![
        40, // "("
        41, // ")"
        60, // "<"
        62, // ">"
        64, // "@"
        44, // ","
        59, // ";"
        58, // ":"
        92, // "\\"
        34, // "\""
        47, // "/"
        91, // "["
        93, // "]"
        63, // "?"
        61, // "="
        123, // "{"
        125, // "}"
        SP,
        HT
    ];

    for i in ts {
        if i == u {
            return false;
        }
    }
    return false;
}

//fn tokenize(content: Vec<u8>)
//

#[derive(PartialEq, Clone, Debug)]
enum Version {
    V0_9,
    V1_0,
    V1_1,
}

#[derive(PartialEq, Clone, Debug)]
enum Method {
    NONE,
    GET,
    HEAD,
    POST,
}

#[derive(Clone, Debug)]
pub struct Request {
    //bytes: Cursor<Vec<u8>>,
    bytes: Vec<u8>,

    method: Method,
    uri: String,
    version: Version,
    ver_str: String,

    rest: String,

    idx: usize,
    space_count: u32,
    terminated: bool,
}

pub fn new() -> Request {
    Request {
        //bytes: Cursor::new(Vec::new()),
        bytes: Vec::new(),
        method: Method::NONE,
        uri: String::new(),
        version: Version::V0_9,
        ver_str: String::new(),
        
        rest: String::new(),

        idx: 0,
        space_count: 0,
        terminated: false,
    }
}

impl Request {

    /*
    pub fn is_terminated(& self) -> bool {
        return self.terminated;
    }


    fn back(&mut self, l: usize) {
        self.idx -= l;
    }

    fn next(&mut self, l: usize) -> Option<&str> {
        if self.bytes.len() < self.idx + l {
            return None;
        }
        let s = Some(std::str::from_utf8(&self.bytes[self.idx..self.idx+l]).unwrap());
        self.idx += l;
        return s;
    }
    */

    fn skip_space(&mut self) {
        let mut length = 0;
        while self.idx + length < self.bytes.len() {
            match self.bytes[self.idx + length] {
                //SP | CR | LF => {
                SP => {
                    length+=1;
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
                    length +=1;
                }
            }
        }

        match std::str::from_utf8(&self.bytes[self.idx..self.idx+length]) {
            Ok(s) => {
                self.idx += length;
                Some(s)
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    fn parse_header_entry(&mut self) -> Option<(&str, &str)> {

        None
        //Some(("", ""))
        //let mut length = 0;
        //while self.idx + length < self.bytes.len() {
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
                self.method = Method::GET;
            }
            Some("HEAD") => {
                self.method = Method::HEAD;
            }
            Some("POST") => {
                self.method = Method::POST;
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
                    self.terminated = true;
                    return Ok(());
                }
                a => {
                    return Err(format!("invalid token: {:?}", a));
                }
        }

        if self.idx < self.bytes.len() {
            for b in self.bytes[self.idx..].iter() {
                self.rest.push(char::from(*b));
            }
        }

        // FIX
        self.terminated = true;

        println!("debug: {:?}", self);
        return Ok(());
    }
    
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
