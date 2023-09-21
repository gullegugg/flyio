use std::io::StdinLock;

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageBody {
    msg_id: i32,
    in_reply_to: i32,
    #[serde(flatten)]
    body: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageContent {
    Init,
    InitOk,
}

pub fn run() {
    while let Some(message) = from_reader::<StdinLock, Message>(std::io::stdin().lock()).ok() {
        let result = handle_message(&message.body.body);
        for message_content in result.output {
            let return_message = Message {
                src: 
            }
            println!("{}", to_string(&message).unwrap());
        }
    }
}

struct HandleResult {
    output: Vec<MessageContent>,
}

fn handle_message(message: &MessageContent) -> HandleResult {
    match message {
        MessageContent::Init => HandleResult {
            output: vec![MessageContent::InitOk],
        },
        MessageContent::InitOk => todo!(),
    }
}
