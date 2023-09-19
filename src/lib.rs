use std::io::StdinLock;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Debug, Serialize, Deserialize)]
struct Message<Body> {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Messages {
    #[serde(rename = "init")]
    Init {},
}

pub fn run() {
    while let Some(message) =
        from_reader::<StdinLock, Message<Messages>>(std::io::stdin().lock()).ok()
    {}
}
