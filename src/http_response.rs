use crate::http_request::*;
use crate::method;
use chrono::Local;
use std::collections::HashMap;

#[allow(dead_code)]
pub mod status {
    pub type Code = (isize, &'static str);
    pub type Code3 = Code;

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
}

#[derive(Clone, Debug)]
pub struct Response {
    pub version: Version,
    pub status: status::Code,
    pub host: &'static str,
    pub path: String,
    pub header: HashMap<String, String>,

    pub entity_body: Vec<u8>,
}

pub fn new() -> Response {
    let mut res = Response {
        version: Version::V0_9,
        status: status::OK,
        host: "",
        path: String::new(),
        header: HashMap::new(),

        entity_body: Vec::new(),
    };

    let date_str = Local::now().to_rfc2822();
    res.add_header("Date", format!("{} GMT", &date_str[..date_str.len() - 6]));
    return res;
}

impl Response {
    pub fn add_header(&mut self, name: &str, value: String) {
        self.header.insert(name.to_string(), value);
    }

    fn status_line(&self) -> String {
        // Status-Line = HTTP-Version SP Status-Code SP Reason-Phrase CRLF
        //               "HTTP/" 1*DIGIT "." 1*DIGIT SP 3DIGIT SP
        format!(
            "HTTP/{} {}\r\n",
            self.version.to_string(),
            status::to_string(self.status)
        )
    }

    #[allow(dead_code)]
    pub fn set_location(&mut self, status: status::Code3, absolute_uri: String) {
        // Location       = "Location" ":" absoluteURI
        self.status = status;
        self.header.insert("Location".to_string(), absolute_uri);
    }

    #[allow(dead_code)]
    pub fn set_extention_status(&mut self, id: isize, phrase: &'static str) {
        self.status = (id, phrase);
    }

    #[allow(dead_code)]
    pub fn allow(&mut self, m: method::Method) {
        let value = match self.header.get("Allow") {
            Some(s) => format!("{}, {}", s, m),
            None => m.to_string(),
        };

        self.header.insert("Allow".to_string(), value);
    }

    pub fn set_host(&mut self, ip_port: String) {
        self.header.insert("HOST".to_string(), ip_port);
    }

    pub fn set_server(&mut self, name: String) {
        // Server         = "Server" ":" 1*( product | comment )
        self.header.insert("Server".to_string(), name);
    }

    // TODO WWW-Authenticate

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = Vec::new();

        // Status-Line
        ret.append(&mut Vec::from(self.status_line().as_bytes()));

        // Header
        for (k, v) in &self.header {
            ret.append(&mut Vec::from(format!("{}: {}\r\n", k, v).as_bytes()));
        }

        ret.append(&mut Vec::from("\r\n".as_bytes()));

        ret.append(&mut self.entity_body.clone());

        return ret;
    }
}
