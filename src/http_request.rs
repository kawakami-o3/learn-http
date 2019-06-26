
//use std::io::Cursor;

//const G: u8 = 71;
//const E: u8 = 69;
//const T: u8 = 84;
const SP: u8 = 32;
const CR: u8 = 13;
const LF: u8 = 10;

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
        let mut l = 0;
        while self.idx + l < self.bytes.len() {
            match self.bytes[self.idx + l] {
                //SP | CR | LF => {
                SP => {
                    l+=1;
                }
                _ => {
                    break;
                }
            }
        }
        self.idx += l;
    }

    fn next_word(&mut self) -> Option<&str> {
        self.skip_space();
        let mut l = 1;
        if self.idx + l >= self.bytes.len() {
            return None;
        }
        while self.idx + l < self.bytes.len() {
            match self.bytes[self.idx + l] {
                SP => {
                    break;
                }
                CR => {
                    if l == 1 {
                        l+=1;
                    } else {
                        break;
                    }
                }
                LF => {
                    l+=1;
                    break;
                }
                _ => {
                    l+=1;
                }
            }
        }


        println!("1> {} {} {}", self.bytes.len(), self.idx, l);
        match std::str::from_utf8(&self.bytes[self.idx..self.idx+l]) {
            Ok(s) => {
                self.idx += l;
                Some(s)
            }
            Err(e) => {
                panic!(e);
            }
        }
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
