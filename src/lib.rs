use error::Error;
use message::{Message, MessageContent};

use crate::message::{Echo, MessageBody};

pub mod error;
mod message;

pub fn run() -> Result<(), Error> {
    for line_result in std::io::stdin().lines() {
        let line = line_result?;
        let input_message: Message = serde_json::from_str(&line)?;

        match input_message.body.body {
            MessageContent::Init(_init) => {
                let response = Message {
                    src: input_message.dest,
                    dest: input_message.src,
                    body: MessageBody {
                        msg_id: None,
                        in_reply_to: input_message.body.msg_id,
                        body: MessageContent::InitOk,
                    },
                };
                println!("{}", serde_json::to_string(&response)?);
            }
            MessageContent::Echo(echo) => {
                let response = Message {
                    src: input_message.dest,
                    dest: input_message.src,
                    body: MessageBody {
                        msg_id: Some(1),
                        in_reply_to: input_message.body.msg_id,
                        body: MessageContent::EchoOk(Echo { echo: echo.echo }),
                    },
                };
                println!("{}", serde_json::to_string(&response)?);
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
