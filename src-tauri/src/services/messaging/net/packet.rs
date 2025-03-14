use std::net::SocketAddr;

use crate::services::messaging::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Packet {
    Message(Message),
    MessageReceived(i32),
}

#[derive(Debug, Clone)]
pub struct ReceivedPacket {
    pub remote: SocketAddr,
    pub packet: Packet,
}
