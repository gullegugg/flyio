use std::io::StdinLock;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageContent<'a> {
    Init {
        node_id: &'a str,
        node_ids: Vec<&'a str>,
    },
    InitOk,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageBody<'a> {
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    #[serde(flatten)]
    body: MessageContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message<'a> {
    src: &'a str,
    dest: &'a str,
    body: MessageBody<'a>,
}

trait Publisher {
    fn publish(&self, message: &Message<'_>);
}

struct JsonSender;

impl Publisher for JsonSender {
    fn publish(&self, message: &Message) {
        let str_val = serde_json::to_string(message).unwrap();
        println!("{}", str_val)
    }
}

struct Node<P: Publisher> {
    node_id: Option<String>,
    send_message: P,
}

impl<P: Publisher> Node<P> {
    fn new(sender: P) -> Self {
        Node {
            node_id: None,
            send_message: sender,
        }
    }

    fn handle_message(&mut self, message: Message) {
        let content: Option<MessageContent> = match message.body.body {
            MessageContent::Init {
                node_id,
                node_ids: _,
            } => {
                self.node_id = Some(node_id.to_string());
                Some(MessageContent::InitOk)
            }
            MessageContent::InitOk => {
                eprintln!("Why did i receive init ok?");
                None
            }
        };
        if let Some(response) = content {
            self.respond_message(&message, response);
        }
    }

    fn respond_message(&self, origin: &Message, message_content: MessageContent) {
        self.send_message.publish(&Message {
            src: &self.node_id.unwrap(),
            dest: origin.src.clone(),
            body: MessageBody {
                msg_id: None,
                in_reply_to: origin.body.msg_id,
                body: message_content,
            },
        })
    }
}

struct CollectPublisher<'a, 'b: 'a> {
    published_messages: Vec<&'a Message<'b>>,
}

impl CollectPublisher<'_, '_> {
    fn new() -> Self {
        CollectPublisher {
            published_messages: Vec::new(),
        }
    }
}

impl<'a, 'b> Publisher for &mut CollectPublisher<'a, 'b> {
    fn publish(&self, message: &Message) {
        self.published_messages.push(message);
    }
}

pub fn run() {
    let mut node = Node::new(JsonSender);
    loop {
        let message = from_reader::<StdinLock, Message>(std::io::stdin().lock()).unwrap();
        node.handle_message(message);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_init() {
        // Given
        let mut publisher = CollectPublisher::new();
        let mut node = Node::new(&mut publisher);

        let node_id = "n2".to_string();

        let message = Message {
            src: "n1".to_string(),
            dest: "n2".to_string(),
            body: MessageBody {
                msg_id: Some(1),
                in_reply_to: None,
                body: MessageContent::Init {
                    node_id: node_id.clone(),
                    node_ids: vec!["n1".to_string(), "n2".to_string()],
                },
            },
        };
        // When
        node.handle_message(message);

        // Then
        assert_eq!(node.node_id, Some(node_id));
        assert_eq!(publisher.published_messages.len(), 1);
    }
}
