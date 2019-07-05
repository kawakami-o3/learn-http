
use crate::http_request::*;

pub mod status {
    pub type Code = (isize, &'static str);

    pub fn to_string(c: Code) -> String {
        format!("{} {}", c.0, c.1)
    }

    pub const OK: Code = (200, "OK");
    pub const CREATED: Code = (201, "Created");
    pub const ACCEPTED: Code = (202, "Accepted");
    pub const NO_CONTENT: Code = (204, "No Content");
    pub const MOVED_PERMANENTLY: Code = (301, "Moved Permanently");
    pub const MOVED_TEMPORARILY: Code = (302, "Moved Temporarily");
    pub const NOT_MODIFIED: Code = (304, "Not Modified");
    pub const BAD_REQUEST: Code = (400, "Bad Request");
    pub const UNAUTHORIZED: Code = (401, "Unauthorized");
    pub const FORBIDDEN: Code = (403, "Forbidden");
    pub const NOT_FOUND: Code = (404, "Not Found");
    pub const INTERNAL_SERVER_ERROR: Code = (500, "Internal Server Error");
    pub const NOT_IMPLEMENTED: Code = (501, "Not Implemented");
    pub const BAD_GATEWAY: Code = (502, "Bad Gateway");
    pub const SERVICE_UNAVAILABLE: Code = (503, "Service Unavailable");
}

#[derive(Clone, Debug)]
pub struct Response {
    pub version: Version,
    pub status: status::Code,

    pub entity_body: String,
}

pub fn new() -> Response {
    Response {
        version: Version::V0_9,
        status: status::OK,

        entity_body: String::new(),
    }
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = String::new();

        // Status-Line
        ret.push_str(format!("HTTP/{} {}\r\n", self.version.to_string(), status::to_string(self.status)).as_str());

        ret.push_str("\r\n");

        ret.push_str(self.entity_body.as_str());

        return Vec::from(ret.as_bytes());
    }
}