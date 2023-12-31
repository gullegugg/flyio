use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Init<'a> {
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Echo<'a> {
    pub echo: &'a str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GenerateOk<'a> {
    pub id: &'a str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Broadcast {
    pub message: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReadOk {
    pub messages: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Topology<'a> {
    #[serde(borrow)]
    pub topology: HashMap<&'a str, Vec<&'a str>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageContent<'a> {
    #[serde(borrow)]
    Init(Init<'a>),
    InitOk,
    Echo(Echo<'a>),
    EchoOk(Echo<'a>),
    Generate,
    GenerateOk(GenerateOk<'a>),
    Broadcast(Broadcast),
    BroadcastOk,
    Read,
    ReadOk(ReadOk),
    Topology(Topology<'a>),
    TopologyOk,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageBody<'a> {
    pub msg_id: Option<i32>,
    pub in_reply_to: Option<i32>,
    #[serde(flatten)]
    #[serde(borrow)]
    pub body: MessageContent<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Message<'a> {
    pub src: &'a str,
    pub dest: &'a str,
    pub body: MessageBody<'a>,
}
