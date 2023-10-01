use std::collections::HashSet;

use error::Error;
use message::{GenerateOk, Message, MessageContent, ReadOk};

use crate::message::{Echo, MessageBody};

pub mod error;
mod message;

fn respond_to(node_id: &str, message: &Message, content: MessageContent) -> Result<(), Error> {
    let response = Message {
        src: node_id,
        dest: message.src,
        body: MessageBody {
            msg_id: Some(1),
            in_reply_to: message.body.msg_id,
            body: content,
        },
    };
    println!("{}", serde_json::to_string(&response)?);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let mut node_id = String::new();
    let mut id_counter: u64 = 0;
    let mut messages: HashSet<i32> = HashSet::new();
    for line_result in std::io::stdin().lines() {
        let line = line_result?;
        let input_message: Message = serde_json::from_str(&line)?;

        match &input_message.body.body {
            MessageContent::Init(init) => {
                node_id = init.node_id.to_string();
                respond_to(&node_id, &input_message, MessageContent::InitOk)?;
            }
            MessageContent::Echo(echo) => {
                respond_to(
                    &node_id,
                    &input_message,
                    MessageContent::EchoOk(Echo { echo: echo.echo }),
                )?;
            }
            MessageContent::Generate => {
                respond_to(
                    &node_id,
                    &input_message,
                    MessageContent::GenerateOk(GenerateOk {
                        id: &format!("{}{}", node_id, id_counter),
                    }),
                )?;
                id_counter += 1;
            }
            MessageContent::Broadcast(broadcast) => {
                messages.insert(broadcast.message);
                respond_to(&node_id, &input_message, MessageContent::BroadcastOk)?;
            }
            MessageContent::Read => {
                respond_to(
                    &node_id,
                    &input_message,
                    MessageContent::ReadOk(ReadOk {
                        messages: messages.iter().map(|i| *i).collect(),
                    }),
                )?;
            }
            MessageContent::Topology(_) => {
                respond_to(&node_id, &input_message, MessageContent::TopologyOk)?;
            }
            _ => {
                return Err(Error::InputError(format!(
                    "Invalid input: {:?}",
                    input_message
                )))
            }
        }
    }
    Ok(())
}

/*
Topology.

Dumb idea:

Store values and store what nodes it has received them from.
Then on incoming message simply send it forward to all nodes it has not gotten it from yet.

n1, n2, n3

? -> n1;
n1 -> n2;
n1 -> n3;
n2 -> n3;
n3 -> n2;

*/
