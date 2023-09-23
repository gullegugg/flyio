use std::{fmt::Error, io::StdinLock};

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
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    #[serde(flatten)]
    body: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageContent {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub fn run() {
    let mut node_id = "".to_string();
    let mut all_nodes: Vec<String> = vec![];

    loop {
        let message = from_reader::<StdinLock, Message>(std::io::stdin().lock()).unwrap();
        match message.body.body {
            MessageContent::Init {
                node_id: new_node_id,
                node_ids: new_node_ids,
            } => {
                node_id = new_node_id;
                all_nodes = new_node_ids;
                let return_message = Message {
                    src: node_id.clone(),
                    dest: message.src.clone(),
                    body: MessageBody {
                        msg_id: None,
                        in_reply_to: None,
                        body: MessageContent::InitOk,
                    },
                };
                match serde_json::to_string(&return_message) {
                    Ok(string) => println!("{}", string),
                    Err(err) => eprintln!("{}", err),
                }
            }
            MessageContent::InitOk => eprintln!("Init failed"),
        }
    }
}

// Börjar helt stateless
// Init ger state: nodes och node id
// Kan ha som map? Eller lättast med bara variabler i struct.
// Stöd för transactionellt? Eller lättast att bara gör allt direkt.
// Vore bra med abstraction över skicka meddelande och loggning.

// Kan också se init som en helt annan grej. Då vet vi att det alltid finns ett node id.

struct Node<MessageSender: Fn(Message)> {
    node_id: Option<String>,
    send_message: MessageSender,
}

impl<MessageSender: Fn(Message)> Node<MessageSender> {
    fn new(sender: MessageSender) -> Self {
        Node {
            node_id: None,
            send_message: sender,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_init() {
        // Given
        let mut messages: Vec<Message> = vec![];
        let mut node = Node::new(|msg| messages.push(msg));
    }
}
