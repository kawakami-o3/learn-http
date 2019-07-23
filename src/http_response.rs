use std::collections::HashMap;

use crate::http_request::*;
use crate::method;
use crate::status;

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
    Response {
        version: Version::V0_9,
        status: status::OK,
        host: "",
        path: String::new(),
        header: HashMap::new(),

        entity_body: Vec::new(),
    }
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
