
//use std::io::Cursor;

//const G: u8 = 71;
//const E: u8 = 69;
//const T: u8 = 84;
const SP: u8 = 32;
//const CR: u8 = 13;
//const LF: u8 = 10;

//fn tokenize(content: Vec<u8>)

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

        idx: 0,
        space_count: 0,
        terminated: false,
    }
}

impl Request {
    pub fn is_terminated(& self) -> bool {
        return self.terminated;
    }

    pub fn parse(&mut self, content: &mut Vec<u8>) {

        self.bytes.append(content);

        if self.bytes.len() < 4 {
            return;
        }

        if self.method == Method::NONE {
            if "GET".as_bytes() == &self.bytes[0..3] {
                self.method = Method::GET;
                self.idx = 3;
            } else if "HEAD".as_bytes() == &self.bytes[0..4] {
                self.method = Method::HEAD;
                self.idx = 4;
            } else if "POST".as_bytes() == &self.bytes[0..4] {
                self.method = Method::POST;
                self.idx = 4;
            } else {
                return;
            }
        }

        if self.space_count == 0 {
            if SP == self.bytes[self.idx] {
                self.idx += 1;
                self.space_count += 1;
            }
        }

        while !(self.space_count >= 2 || self.terminated) {
            if "\r\n".as_bytes() == &self.bytes[self.idx..self.idx+2] {
                // HTTP/0.9
                self.terminated = true;
                self.idx += 2;
            } else if SP == self.bytes[self.idx] {
                // HTTP/1.0
            } else {
                self.uri.push(char::from(self.bytes[self.idx]));
                self.idx += 1;
            }
        }

        println!("{:?}", self);

        if self.space_count >= 2 {
            // HTTP/1.0
        }
    }
    
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
