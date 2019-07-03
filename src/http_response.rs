
use crate::http_request::*;

#[derive(Clone, Debug)]
pub struct Response {
    pub version: Version,
}

pub fn new() -> Response {
    Response {
        version: Version::V0_9,
    }
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = String::new();
        ret.push_str(format!("HTTP/{} 200 ", self.version.to_string()).as_str());
        ret.push_str("\r\n");
        return Vec::from(ret.as_bytes());
    }
}
