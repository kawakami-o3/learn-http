
use crate::http_request::*;

pub mod status {
    pub type Code = isize;

    pub const OK: Code = 200;
    pub const CREATED: Code = 201;
    pub const ACCEPTED: Code = 202;
    pub const NO_CONTENT: Code = 204;
    pub const MOVED_PERMANENTLY: Code = 301;
    pub const MOVED_TEMPORARILY: Code = 302;
    pub const NOT_MODIFIED: Code = 304;
    pub const BAD_REQUEST: Code = 400;
    pub const UNAUTHORIZED: Code = 401;
    pub const FORBIDDEN: Code = 403;
    pub const NOT_FOUND: Code = 404;
    pub const INTERNAL_SERVER_ERROR: Code = 500;
    pub const NOT_IMPLEMENTED: Code = 501;
    pub const BAD_GATEWAY: Code = 502;
    pub const SERVICE_UNAVAILABLE: Code = 503;
}

#[derive(Clone, Debug)]
pub struct Response {
    pub version: Version,
    pub status: status::Code,
}

pub fn new() -> Response {
    Response {
        version: Version::V0_9,
        status: status::OK,
    }
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = String::new();

        // Status-Line
        ret.push_str(format!("HTTP/{} {} \r\n", self.version.to_string(), self.status).as_str());

        ret.push_str("\r\n");
        return Vec::from(ret.as_bytes());
    }
}
