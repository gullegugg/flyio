use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Init<'a> {
    node_id: &'a str,
    node_ids: Vec<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Echo<'a> {
    echo: &'a str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageContent<'a> {
    #[serde(borrow)]
    Init(Init<'a>),
    InitOk,
    Echo(Echo<'a>),
    EchoOk(Echo<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MessageBody<'a> {
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    #[serde(flatten)]
    #[serde(borrow)]
    body: MessageContent<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Message<'a> {
    src: &'a str,
    dest: &'a str,
    body: MessageBody<'a>,
}

#[derive(Debug)]
pub enum Error {
    InputError(String),
    InvalidMessage(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::InputError(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::InputError(value.to_string())
    }
}

pub fn run() -> Result<(), Error> {
    for line in std::io::stdin().lines() {
        let line_thing = &line?;
        let input_message: Message = serde_json::from_str(line_thing)?;

        match input_message.body.body {
            MessageContent::Init(init) => {
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
                eprintln!("{:?}", init)
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
                eprintln!("{:?}", echo)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde() {
        // Given
        let msg = Message {
            src: "n1",
            dest: "n2",
            body: MessageBody {
                msg_id: Some(1),
                in_reply_to: Some(2),
                body: MessageContent::Init(Init {
                    node_id: "n2",
                    node_ids: vec!["n1", "n2"],
                }),
            },
        };

        // When deserialize and serialize
        let ser = serde_json::to_string(&msg).unwrap();
        let de: Message = serde_json::from_str(&ser).unwrap();

        // Then
        assert_eq!(msg, de);
    }
}
