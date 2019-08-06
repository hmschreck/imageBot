extern crate clipboard;
extern crate reqwest;
extern crate regex;
use std::{thread, time};

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::collections::HashMap;
use regex::Regex;
#[macro_use] extern crate lazy_static;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut x = String::new();
    let mut old = String::new();
    loop {
        x = ctx.get_contents().unwrap().to_string();
        if x != old {
            if is_image_link(x.to_string()) {
                send(x.to_string());
                old = x;
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn is_image_link(link: String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("[jpg|jpeg|png|gif]$").unwrap();
    }
    return RE.is_match(link.as_str());
}

fn send(link: String) {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("text", link);
    client.post("Put it here.  Lazy right now.")
        .json(&map)
        .send();
}
