
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

thread_local! {
    static CONF: RefCell<ServerConf> = RefCell::new(new_conf());
}

pub fn set(conf: ServerConf) {
    CONF.with(|c| {
        *c.borrow_mut() = conf;
    })
}

pub fn port() -> String {
    CONF.with(|c| {
        c.borrow().port.clone()
    })
}

#[derive(Clone, Deserialize)]
pub struct ServerConf {
    pub port: String,
}

fn new_conf() -> ServerConf {
    ServerConf{
        port: String::new(),
    }
}

pub fn load(path: &str) -> ServerConf {
    let mut buffer = String::new();
    match File::open(path) {
        Ok(mut file) => {
            match file.read_to_string(&mut buffer) {
                Ok(_) => { }
                Err(e) => {
                   panic!(e);
                }
            }
        }
        Err(e) => {
            panic!(e);
        }
    };

    match serde_json::from_str(buffer.as_str()) {
        Ok(conf) => {
            //CONF.with(|c| { *c.borrow_mut() = conf.clone(); });
            return conf;
        }
        Err(e) => {
            panic!(e);
        }
    }
}


