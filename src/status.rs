#![allow(dead_code)]

pub type Code = (isize, &'static str);
pub type Code3 = Code;

pub fn ext(i: isize, s: &'static str) -> Code {
    (i, s)
}

pub fn to_string(c: Code) -> String {
    format!("{} {}", c.0, c.1)
}

pub const OK: Code = (200, "OK");
pub const CREATED: Code = (201, "Created");
pub const ACCEPTED: Code = (202, "Accepted");
pub const NO_CONTENT: Code = (204, "No Content");
pub const MOVED_PERMANENTLY: Code3 = (301, "Moved Permanently");
pub const MOVED_TEMPORARILY: Code3 = (302, "Moved Temporarily");
pub const NOT_MODIFIED: Code3 = (304, "Not Modified");
pub const BAD_REQUEST: Code = (400, "Bad Request");
pub const UNAUTHORIZED: Code = (401, "Unauthorized");
pub const FORBIDDEN: Code = (403, "Forbidden");
pub const NOT_FOUND: Code = (404, "Not Found");
pub const INTERNAL_SERVER_ERROR: Code = (500, "Internal Server Error");
pub const NOT_IMPLEMENTED: Code = (501, "Not Implemented");
pub const BAD_GATEWAY: Code = (502, "Bad Gateway");
pub const SERVICE_UNAVAILABLE: Code = (503, "Service Unavailable");

