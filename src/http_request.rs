#![feature(advanced_slice_patterns, slice_patterns)]

const G: u8 = 71;
const E: u8 = 69;
const T: u8 = 84;
const SP: u8 = 32;
const CR: u8 = 13;
const LF: u8 = 10;

//fn tokenize(content: Vec<u8>)


#[derive(Clone, Debug)]
pub struct HttpRequest {
}

pub fn parse(content: Vec<u8>) -> HttpRequest {
    //let cnt = String::from_utf8(content.to_vec());
    //let cnt = String::from_utf8(content);
/*
    match content.as_slice() {
        [G, E, T, SP, ref inside .. , CR, LF] => {
        //[G, E, T, SP, rest] => {
            println!("{:?}", inside);
        }
        _ => {
            println!("error");
        }
    }
    */
    HttpRequest{
    }
}
